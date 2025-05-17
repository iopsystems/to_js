// Wrap a WebAssembly.Instance, returning an object containing its #[js] exports.
// If the optional second argument is true, typed arrays (including ones that
// were stashed or returned as packed arrays) will always be copied out of the
// WebAssembly heap before being returned.
export function wrapInstance(instance, alwaysCopyData) {
	// In enum variant order (enum: ArrayType)
	const arrayTypes = [
		Uint8Array,
		Int8Array,
		Uint16Array,
		Int16Array,
		Uint32Array,
		Int32Array,
		Float32Array,
		BigUint64Array,
		BigInt64Array,
		Float64Array,
	];

	const f64Array = new Float64Array(1);
	const typedArrays = arrayTypes.map(T => new T(f64Array.buffer));

	function asArray(x, i) {
		f64Array[0] = x;
		return typedArrays[i];
	}

	const u8Octet = (x) => asArray(x, 0);
	const i8Octet = (x) => asArray(x, 1);
	const u16Quartet = (x) => asArray(x, 2);
	const i16Quartet = (x) => asArray(x, 3);
	const u32Pair = (x) => asArray(x, 4);
	const i32Pair = (x) => asArray(x, 5);
	const f32Pair = (x) => asArray(x, 6);
	const asU64 = (x) => asArray(x, 7)[0];
	const asI64 = (x) => asArray(x, 8)[0];

	const textDecoder = new TextDecoder();
	const instanceExports = instance.exports;

	// In enum variant order (enum: Transform)
	const outputTransforms = [
		u8Octet,
		i8Octet,
		u16Quartet,
		i16Quartet,
		u32Pair,
		i32Pair,
		f32Pair,
		asU64,
		asI64,
		(x) => x,
		(x) => {},
		Boolean,
		(x) => textDecoder.decode(x),
		(x) => JSON.parse(textDecoder.decode(x)),
	];

	function cString(ptr) {
		const bytes = new Uint8Array(instanceExports.memory.buffer, ptr);
		const end = bytes.findIndex((d) => d === 0);
		return textDecoder.decode(bytes.subarray(0, end));
	}

	function throwError(ptr) {
		throw new Error(cString(ptr));
	}

	// Implement decoding for both niche strategies
	
	function tryResultHighBitsNaN(pair) {
		if (pair[0] !== 1 && pair[1] === 0xfff80000) {
			throwError(pair[0]);
		}
	}

	function tryResultLowBitsOne(pair) {
		if (pair[0] === 1 && pair[1] !== 0) {
			throwError(pair[1]);
		}
	}

	function tryOptionHighBitsNaN(pair) {
		return pair[0] === 0 && pair[1] === 0xfff80000;
	}

	function tryOptionLowBitsOne(pair) {
		return pair[0] === 1 && pair[1] === 0;
	}

	function tryOption(isArray) {
		return isArray ? tryOptionLowBitsOne : tryOptionHighBitsNaN;
	}

	function tryResult(isArray) {
		return isArray ? tryResultLowBitsOne : tryResultHighBitsNaN;
	}

	return Object.fromEntries(
		Object.keys(instanceExports)
			.filter((d) => d.endsWith("_info_"))
			.map((nameWithSuffix) => {
				const name = nameWithSuffix.slice(0, -6);
				const typeInfo = u8Octet(instanceExports[`${name}_info_`]());
				const [isResult, isOption, isArray, arrayType, transformIndex] = typeInfo;
				const numArgs = instanceExports[name].length;
				const args = Array.from({ length: numArgs }, (_, i) => `x${i + 1}`);
				const argsAsString = args.join(", ");
				const needsPair = isResult || isOption || isArray;
				const isPackedArray = transformIndex < 7;
				const isIdentityTransform = transformIndex === 9;
				const slice = alwaysCopyData && (isPackedArray || (isArray && isIdentityTransform));
				// Compile a specialized function for each export using basic dead-code elimination to elide
				// unnecessary transformations (eg. only include Option-processing code if the return value is an Option).
				const fn = new Function(`exports`, `tryResult`, `tryOption`, `transform`, `u32Pair`, `
					return function ${name}(${argsAsString}) {
						if (arguments.length !== ${args.length}) {
							throw new Error(\`${name}: expected ${args.length} argument${args.length === 1 ? '' : 's'}, got \${arguments.length}\`);
						}
						let value = exports.${name}(${argsAsString});
						${needsPair ? `const pair = u32Pair(value);` : ``}
						${isResult ? `tryResult(pair);` : ``}
						${isOption ? `if (tryOption(pair)) return null;` : ``}
						${isArray ? `value = new ${arrayTypes[arrayType].name}(exports.memory.buffer, pair[0], pair[1])` : ``}
						const ret = transform(value);
						return ${slice ? `ret.slice()` : `ret`}
					}`);
				return [
					name,
					fn(
						instanceExports,
						tryResult(isArray),
						tryOption(isArray),
						outputTransforms[transformIndex],
						u32Pair
					)
				];
			})
	);
}

// Create a JavaScript-side class that corresponds to a Rust-side struct.
export function createClass(
	// A WebAssembly instance wrapper returned by `wrapInstance(instance)`
	instance,
	// Name prefix shared by all methods, separated from method names by an underscore
	prefix,
	{
		// Optional array of method names. Will be inferred from the prefix if not provided.
		methods,
		// Optional Object from method name to wrapper function that can transform the return value of a method.
		transforms
	} = {}
) {
	// Ensure the prefix ends with an underscore
	if (!prefix.endsWith("_")) {
		prefix += "_";
	}

	// The constructor function. We assume this is named alloc, to go with dealloc.
	const allocMethod = prefix + "alloc";
	const alloc = instance[allocMethod];
	if (!(allocMethod in instance)) throw new Error('Missing constructor: ' + allocMethod);

	const deallocMethod = prefix + "dealloc";
	if (!(deallocMethod in instance)) throw new Error('Missing destructor: ' + deallocMethod);

	// Infer methods based on prefix if not provided.
	// This can misfire if the prefix for this class is a prefix of
	// another, longer class prefix (eg. foo_xxx will catch foo_bar_xxx).
	if (methods === undefined) {
		methods = [];
		for (const name in instance) {
			if (name.startsWith(prefix) && name !== allocMethod) {
				methods.push(name.slice(prefix.length));
			}
		}
	}

	// Ensure that "dealloc" is a method on the class
	if (!methods.includes("dealloc")) {
		methods.push("dealloc");
	}

	// Create the constructor function and add method definitions to its prototype
	const Class = function (...args) {
		this.ptr = alloc(...args);
	};

	const identity = (x) => x;

	for (const name of methods) {
		const method = instance[prefix + name];
		if (method === undefined) {
			throw new Error("undefined method: " + (prefix + name));
		}
		const transform = transforms?.[name] ?? identity;
		Class.prototype[name] = function (...args) {
			return transform(method(this.ptr, ...args));
		};
	}
	return Class;
}