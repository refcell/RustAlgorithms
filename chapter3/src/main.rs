#![allow(unused)]

// * Constants
const MAX_POINTS: u32 = 100_000;

fn main() {
    // * Variable Mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // * Variable Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // * Variable re-declaration with `let`
    let spaces = "   ";
    let spaces = spaces.len();

    // * Tuples
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // * Array ops
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let first = a[0];
    let second = a[1];
    println!("The value of second is: {}", second);

    // * Functions
    a_nother_function();
    another_function(1, 2);
    another_another_function(10);

    // * Blocks
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    // * Return functions
    println!("The value of x is: {}", five());
    println!("The value of x + 1 is: {}", plus_one(10));
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn a_nother_function() {
    println!("Another function.");
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn another_another_function(x: i32) {
    println!("The value of x is: {}", x);
}
