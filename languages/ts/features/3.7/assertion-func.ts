declare function assert(value: unknown): asserts value;
const someValue = 42;
//const someValue = 43;
assert(someValue === 42);

function multiply(x, y) {
    assert(typeof x === "number");
    assert(typeof y === "number");

    return x * y;
}

function yell(str: string) {
    assert(typeof str === "string");

    return str.toUpperCase();
    //return str.toUppercase();
    // Oops! We misspelled 'toUpperCase'.
    // Would be great if TypeScript still caught this!
}
