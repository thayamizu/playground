// @ts-check
var Foo = /** @class */ (function () {
    function Foo() {
        /** @readonly */
        this.stuff = 100;
    }
    Foo.prototype.writeToStuff = function () {
        this.stuff = 200;
        //   ~~~~~
        // Cannot assign to 'stuff' because it is a read-only property.
    };
    return Foo;
}());
new Foo().stuff++;
//        ~~~~~
// Cannot assign to 'stuff' because it is a read-only property.
