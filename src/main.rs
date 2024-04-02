fn main() {
    let  x = 5;
    
    let x = x + 1; 
    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }
    println!("The value of x is: {x}");
    
    let spaces = "  ";
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
    data_types();
    array_len();
}


fn data_types() {
    let x = 0.4;   //f64
    let y: f32 = 0.2;   //f32
    println!("{x}, {y}");

    // addition
    let sum = 5 + 10;
    println!("sum {sum}");

    // subtraction
    let difference: f32 = 95.4 - 4.3;
    println!("difference {difference}");

    // multiplication
    let product: f32 = 4.0 * 30.0;
    println!("product {product}");

    // reminder 
    let quotient = 54.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("truncated {truncated}");
    println!("quotient {quotient}");


    //reminder 
    let reminder = 43 % 5;
    println!("reminder {reminder}");

    //booling
    let t = true;
    let f: bool = false;    // with explicit type annotation
    println!("booling {t}, {f}");

    //char
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation;
    let heart_eyed_cat = '😻';
    println!("char {heart_eyed_cat}, {c}, {z}");

    //Tuples 
    //  let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; 
    println!("tuples, {x},{y},{z}");

    let x = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("x.0, {five_hundred}");
    println!("x.1, {six_point_four}");
    println!("x.2, {one}");

    // Array
    let a = [1, 2, 3, 4, 5];
    let b = a[0];
    println!("a[0], {b}"); 

    let a = ["let"; 5];
    let b = a[1];
    println!("array q[4], {b} ");
}

use std::io;

fn array_len() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
