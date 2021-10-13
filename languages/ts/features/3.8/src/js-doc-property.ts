// @ts-check

class Foo {
    constructor() {
        /** @readonly */
        this.stuff = 100;
    }

    writeToStuff() {
        this.stuff = 200;
        //   ~~~~~
        // Cannot assign to 'stuff' because it is a read-only property.
    }
}

new Foo().stuff++;
//        ~~~~~
// Cannot assign to 'stuff' because it is a read-only property.
