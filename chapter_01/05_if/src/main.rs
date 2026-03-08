#![allow(unused)]

fn main() {
    // Conditions are expressions that must evaluate to a boolean type, either resulting in true
    // or false.
    let condition = true;
    let condition = 5 > 10; // False as 5 is not greater than 10.
    let condition = 5 == 5; // True as 5 is equal to 5.

    // If expressions are conditional pieces of code that only execute if evaluated as true. If
    // expressions can contain multiple "branches" which check for different conditions.
    // Declare an if expression using `if` followed by a condition expression, followed by a
    // block expression.
    if 5 == 5 {
        println!("5 is equal to 5");
    }

    // If expressions can contain multiple branches, and a branch can ONLY be executed if its
    // condition evaluates to true. Additionally, only ONE branch can be executed, and that is
    // the first branch whose condition evaluates to true.

    // Branches can be declared using `if`, `else if`, and `else`.
    // - `if` can be used by itself or with other branches and checks for a condition.
    // - `else if` can only be used after an existing `if` or other `else if` branch and also
    // checks for a condition.
    // - `else` does not specify a condition and only executes if none of the other branches'
    // conditions were evaluated as true.
    if 5 == 5 {
        println!("5 is equal to 5"); // Gets executed
    } else if 6 > 3 {
        println!("6 is greater than 3");
    } else {
        println!("Both previous branches were false");
    }

    // In the example above, ONLY the first branch gets executed, even though the second branch
    // also evaluates as true. This is because as soon as Rust sees a true branch, it executes
    // that branch and skips all others.

    // Because if expressions are just normal expressions that evaluate to some value, they can
    // also be assigned to bindings. For example:
    let number = 10;
    let result: i32 = if number == 20 {
        println!("Number is 20");
        20
    } else {
        println!("Number is 10");
        10
    };

    // Here, `number` is 10, and the first condition checks if the value of number is 20. That
    // condition evaluates to false, so the else branch is executed, and returns 10.
    // Each branch MUST result in a consistent type. For example:

    // let number = 10;
    // let result: i32 = if number == 20 {
    //     println!("Number is 20");    mismatched type error
    // } else {
    //     println!("Number is 10");
    //     10
    // };

    // This results in a compilation error since Rust expects the result of an expression to be
    // consistent, and if all branches do not return a consistent type Rust refuses to compile.
    // This is because of the strict type system in Rust. In this example, the first branch would
    // return `()`, and the second branch would return `i32`, which is a conflict.

    // Using a single if branch in Rust is valid ONLY if the result is (). If you expect a result
    // that is anything other than (), an `else` branch is mandatory. For example:

    // let result = if 5 > 3 {
    //     5    // mismatched types expected () found i32
    // };

    // This is because if the condition is false, there is no value to return; not even () can be
    // implicitly returned from this. The correct way is to always specify an `else` branch if
    // you intend on a result with any meaningful value:

    let result = if 3 > 5 {
        "3 is greater than five"
    } else {
        "5 is greater than 3"
    };

    // Because conditions are expressions, they can be written in block expression syntax. This
    // is as long as the block expression returns an expression which evaluates to bool. For
    // example:

    let result = if {
        let x = 100;
        400 > 100 // Evaluates to true because 400 IS greater than 100
    } {
        400
    } else if {
        let x = 200;
        200 > 300 // Evaluates to false because 200 is NOT greater than 300
    } {
        200
    } else {
        100
    };

    // Essentially the syntax is:
    // let <variable> = if { <condition> } { <branch body> } else if { <condition> } { <branch
    // body> } else { <branch body> };
}
