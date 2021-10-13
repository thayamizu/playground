fn main() {
    println!("Hello, world!");
    another_function();
    f1(10);
    f2(10, 20);

    let x = f3();
    println!("x is:{}", x);
}

fn another_function() {
    println!("Another function.");
}

fn f1(x:i32) {
    println!("The value of x is:{}", x);
}

fn f2(x:i32, y:i32) {
    println!("The value of x is:{} y is:{}", x, y);
}

fn f3()->i32 {
    return 5;
}