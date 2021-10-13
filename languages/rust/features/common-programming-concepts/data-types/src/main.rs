fn main() {
    // This code will occur compile error!!
    //let guess: u32 = "42".parse().expect("Not a number!");
    
    //-------------------------------------
    //Scalar Types
    //-------------------------------------
    //integer types
    let v :i8 = 0; //signed 8bit
    let v :u8 = 0; //unsinged 8bit;
    let arch: usize = 0; //usize depends on the kind of computer. 

    //Integer Literal
    let v :u16 = 1_127;
    let v :u8 = 0x_ff;
    let v :u8 = 0o77;
    let v :u8 = 0b1111_0000;
    let v :u8 = b'A';//u8 only

    //-------------------------------------
    //Floating Types
    //-------------------------------------
    let v = 2.0;//f64
    let v: f32 = 3.0;
    
    //-------------------------------------
    //Numeric Operation
    //-------------------------------------
    let s = 5 + 2;
    let s = 5 - 2;
    let s = 5 * 2;
    let s = 5 / 2;
    
    //-------------------------------------
    //Boolean Types
    //-------------------------------------
    let v: bool = true;
    
    //-------------------------------------
    //Character Types
    //-------------------------------------
    let v = 'c';
    let v = 'Z';
    let heart_eyed_cat = '😻';//emoji

    //-------------------------------------
    //Compound Types
    //-------------------------------------
    let tup = (400, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    
    //-------------------------------------
    //Array Types
    //-------------------------------------
    let arr: [i32;5] = [1,2,3,4,5];
    
    let first = arr[0];
    let second = arr[1];
    //This code will occur compile error;
    //let out_of_range = arr[10];

}
