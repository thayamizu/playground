const MAX_POINTS: u32 = 100_000;

fn shadowing() -> i32 {
    let x = 10;
    let x = x + 5;
    x
}

#[test]
fn shadowing_test() {
    let actual = shadowing();
    let expect = 15;
    assert_eq!(actual, expect)
}

fn mut_fun(mut x: i32) -> i32 {
    x = x + 1;
    x
}

#[test]
fn mut_fn_test() {
    let input = 10;
    let actual = mut_fun(input);
    let expect = input + 1;

    assert_eq!(actual, expect);
}

fn main() {
    let x = 10;
    println!("x value is {}", x);

    let x = mut_fun(x);
    println!("x value is {}", x);
}
