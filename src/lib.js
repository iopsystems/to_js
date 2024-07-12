// data:text/javascript, export default
function toJs(instance, alwaysCopyData) {
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
        (x) => new TextDecoder().decode(x),
        (x) => cString(u32Pair(x)[0]),
    ];

    function cString(buffer, ptr) {
        const bytes = new Uint8Array(buffer, ptr);
        const end = bytes.findIndex((d) => d === 0);
        return bytes.subarray(0, end);
    }

    function throwError(ptr) {
        throw new Error(cString(ptr));
    }

    function tryResultNaNStyle(pair) {
        if (pair[0] !== 0 && pair[1] === 0xfff80000) {
            throwError(pair[0]);
        }
    }

    function tryResultNullPtrStyle(pair) {
        if (pair[0] === 0 && pair[1] !== 0) {
            throwError(pair[1]);
        }
    }

    function tryOptionNaNStyle(pair) {
        return pair[0] === 0 && pair[1] === 0xfff80000;
    }

    function tryOptionNullPtrStyle(pair) {
        return pair[0] === 0 && pair[1] === 0;
    }

    const instanceExports = instance.exports;

    return Object.fromEntries(
        Object.keys(instanceExports)
            .filter((d) => d.endsWith("_info_"))
            .map((nameWithSuffix) => {
                const name = nameWithSuffix.slice(0, -6);
                const [isResult, isOption, isArray, arrayType, transform, isMulti] =
                    u8Octet(instanceExports[`${name}_info_`]());
                const numArgs = instanceExports[name].length;
                const args = Array.from({ length: numArgs }, (_, i) => `x${i + 1}`);
                const argsAsString = args.join(", ");
                const needsPair = isResult || isOption || isArray || isMulti;
                const isPackedArray = transform < 7;
                const isIdentityTransform = transform === 9;
                const slice = alwaysCopyData && (isPackedArray || (isArray && isIdentityTransform));
                const fn = new Function(`exports`, `tryResult`, `tryOption`, `transform`, `u32Pair`, `
                return function(${argsAsString}) {
                    if (arguments.length !== ${args.length}) {
                        throw new Error(\`expected ${args.length} arguments, got \${arguments.length}\`);
                    }
                    let value = exports.${name}(${argsAsString});
                    ${needsPair ? `const pair = u32Pair(value);` : ``}
                    ${isResult ? `tryResult(pair);` : ``}
                    ${isOption ? `if (tryOption(pair)) return null;` : ``}
                    const ret = transform(
                        ${isArray
                        ? `new ${arrayTypes[arrayType].name}(exports.memory.buffer, pair[0], pair[1])` 
                        : `value`}
                    );
                    return ${slice ? `ret.slice()` : `ret`}
                }`);
                return [
                    name,
                    fn(
                        instanceExports,
                        isResult && (isArray ? tryResultNullPtrStyle : tryResultNaNStyle),
                        isOption && (isArray ? tryOptionNullPtrStyle : tryOptionNaNStyle),
                        outputTransforms[transform],
                        u32Pair
                    )
                ];
            })
    );
}