#![allow(unused)]

fn main() {
    let number = 4;

    let x = if number < 5 { () } else { () };

    let condition = true;
    let number = if condition { 10 } else { 5 };
    println!("{number}");

    let number: i32 = 10;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2.");
    }
}
