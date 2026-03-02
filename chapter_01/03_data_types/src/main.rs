// Prevent compile-time warnings due to unused 02_variables and constants
#![allow(unused)]

use std::io;

fn main() {
    // Integers in Rust are values without a fractional component. Rust default's to the i32 type
    // for numeric integer types that do not have a decimal point.
    let integer = 5;

    // Integers can either be signed or unsigned and has an explicit size. If a variable is
    // signed (contains a sign in front like "-"), it can either have a negative or positive value.
    // If a variable is unsigned (does not contain a sign in front) then it can only hold positive
    // values.
    let signed_int: i32 = 10; // i (integer) means it is signed
    let unsigned_int: u32 = 15; // u means it is unsigned

    // Floating-point numbers in Rust are numeric values with a decimal point. Rust default's to
    // the f64 type for floating points. This is because 64-bit sizes allow for more precision
    // than 32-bits, and is relatively cheap to do.
    let float = 5.0;
    let float_32: f32 = 4.0; // 32-bit

    // Boolean types can have one of two values, either true or false. Also, booleans can be type
    // annotated with `bool` in the declaration. For example,
    let boolean = true;
    let boolean: bool = false;

    // Char types are Rust's most primitive alphabetic type. They can be type annotated with the
    // `char` keyword. Char literals are defined in single quotation marks, unlike double-quoted
    // strings. Char types are 4 bytes in size and represent a Unicode scalar value.
    let character = 'z';
    let character: char = '';
    let character = '🤑';

    // Compound types in Rust can group multiple values into one single type. Rust has two
    // primitive compound types: Tuple and Array.

    // Tuples are a compound type which can hold a number of types that can be different. They
    // are assigned to values which correspond to the types annotated in the declaration.
    // The type annotation for the tuple is enclosed in parentheses, and so are the values bound
    // to that tuple. For example,
    let tuple: (i32, u64, f32) = (10, 690_000, 67.69);

    // The "destructuring" pattern can be used to separate a tuple into multiple bindings. Each
    // binding's type and value correspond to its respective index in the tuple. Indices in Rust
    // start from zero.

    // You can either destructure using similar tuple syntax with parenthesis all in one line.
    let (x, y, z) = tuple;

    // Or you can destructure each tuple index individually.
    let x = tuple.0; // index 1 (10)
    let y = tuple.1; // index 2 (690,000)
    let z = tuple.2; // index 3 (67.69)

    // Tuples without any values are called "units", written in empty parenthesis. Units have an
    // empty value and are used to describe when there is no return type.
    let unit: () = ();

    // Arrays are a compound type which contain an explicit size (length) of elements which cannot
    // change. Every element in the array will have the same type. Attempting to bind values with
    // different types to the array will result in compile-time errors. Arrays can be declared using
    // square brackets which contains a type and a size, separated by a semicolon:
    // [type;size] = [element, element, ...] . For example,
    let array = [1, 2, 3, 4, 5];

    // You can also declare an array by binding a set value and size: [value;size] . The size you
    // set will initialize all elements in the array with that value. For example,
    let a = [10; 2]; // equivalent to: [10, 10]

    // You can access the elements in the array using "indexing". An index is a number which
    // corresponds to an element in an array. Indexing in Rust starts from zero. For example,
    let first = a[0]; // Bind the value (10) of the second element in array 
    let second = a[1]; // Bind the value (10) of the second element in array

    // When indexing in Rust, the index you use MUST be less than the size (length) of the array.
    // For example, the array `a` has a length of 2 since it contains two elements. Using the
    // index 2 (or greater) would correspond to outside the bounds of the array, making the
    // program crash.
    // let third = a[2]; A third element does not exist

    // EXAMPLE INDEXING PROGRAM:
    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin().read_line(&mut index).unwrap();

    let index: usize = index.trim().parse().unwrap();
    // If the user inputted an index which is outside the bounds of the array, the program will
    // panic here before the result can be printed.
    let element = array[index];

    println!("The value of the element at index {index} is: {element}");
}
