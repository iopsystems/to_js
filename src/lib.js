export default function toJs(instance, alwaysCopyData) {
    // In enum variant order (ArrayType)
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

    // In enum variant order (OutputTransform)
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
        dynamic,
        (x) => {
            const ret = [];
            for (let i = 0; i < x.length; i += 2) {
                ret.push(dynamic(x.slice(i, i + 2)));
            }
            return ret;
        }
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

    function dynamic([value, typeInfo]) {
        const [flags, isArray, arrayType, transform] = u8Octet(typeInfo);
        const isResult = flags & 1, isOption = flags & 2;
        const isPackedArray = transform < 7;
        const isIdentityTransform = transform === 9;
        const slice = alwaysCopyData && (isPackedArray || (isArray && isIdentityTransform));
        const pair = u32Pair(value);
        if (isResult) tryResult(isArray)(pair);
        if (isOption && tryOption(isArray)(pair)) return null;
        if (isArray) value = new arrayTypes[arrayType](instanceExports.memory.buffer, pair[0], pair[1]);
        const outputTransform = outputTransforms[transform];
        const ret = outputTransform(value);
        return slice ? ret.slice() : ret;
    }

    return Object.fromEntries(
        Object.keys(instanceExports)
            .filter((d) => d.endsWith("_info_"))
            .map((nameWithSuffix) => {
                const name = nameWithSuffix.slice(0, -6);
                const typeInfo = u8Octet(instanceExports[`${name}_info_`]());
                const [flags, isArray, arrayType, transform] = typeInfo;
                const isResult = flags & 1, isOption = flags & 2;
                const numArgs = instanceExports[name].length;
                const args = Array.from({ length: numArgs }, (_, i) => `x${i + 1}`);
                const argsAsString = args.join(", ");
                const needsPair = isResult || isOption || isArray;
                const isPackedArray = transform < 7;
                const isIdentityTransform = transform === 9;
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
                        outputTransforms[transform],
                        u32Pair
                    )
                ];
            })
    );
}