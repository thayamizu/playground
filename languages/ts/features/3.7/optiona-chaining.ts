let foo = {
    bar: {
        baz: function () {
            console.log("baz");
            return "baz";
        }
    }
}

let x = foo?.bar.baz();


