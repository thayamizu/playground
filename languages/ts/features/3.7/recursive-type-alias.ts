// For example, this is not feasible in 3.6:
type ValueOrArray<T> = T | Array<ValueOrArray<T>>;

// An implementation would have looked like this, by mixing
// the type with an interface.
type ValueOrArray2<T> = T | ArrayOfValueOrArray<T>;
interface ArrayOfValueOrArray<T> extends Array<ValueOrArray2<T>> { }

// This allows for a comprehensive definition of JSON,
// which works by referring to itself.

type Json = string | number | boolean | null | Json[] | { [key: string]: Json };

const exampleStatusJSON: Json = {
    available: true,
    username: "Jean-loup",
    room: {
        name: "Highcrest",
        // Cannot add functions into the Json type
        // update: () => {}
    }
}