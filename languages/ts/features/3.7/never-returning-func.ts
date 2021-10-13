
type SomeType = never | string | number
function doThingWithString(v: string) {
    return v
}

function doThingWithNumber(v: number) {
    return v
}
function dispatch_1(x: string | number): SomeType {
    if (typeof x === "string") {
        return doThingWithString(x);
    }
    else if (typeof x === "number") {
        return doThingWithNumber(x);
    }
    return process.exit(1);
}

function dispatch_2(x: string | number): SomeType {
    if (typeof x === "string") {
        return doThingWithString(x);
    }
    else if (typeof x === "number") {
        return doThingWithNumber(x);
    }
    process.exit(1);
}