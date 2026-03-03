#![allow(unused)]

// A function is Rust can be declared with the `fn` keyword. This is followed by the function
// name and a parenthesis. The parenthesis can (optionally) hold special variables called
// "parameters" which must be type annotated. These parameters can be used like normal bindings
// with the function.
// The function can also (optionally) specify a "return type" using the `->` operator followed by a
// type. The return type states what the type of the value the ending expression will evaluate to.
// The function body is the last part, used with the `{}` operator. Inside the curly brackets is
// the function body or "definition".

// Here, the name of the function is `five` with no parameter specified. It has a return type of
// `i32`, which means this function MUST return an integer. This function's body contains an
// expression `5 + 5`, which evaluates to `10`. 10 becomes the result, or "return value" of the
// function. The return value's type conforms to the function return type of `i32`.
fn five() -> i32 {
    // Rust is an expression-based-language. An expression in Rust, simply means something that
    // evaluates to a value. The "value" can also be called "result".

    // Here, 5 + 5 is considered an expression, because it evaluates to 10, which is a value.
    // Rust can deduce that the type of the value 10 is a numeric scalar value, which also works
    // with the return type of this function `i32`.
    5 + 5

    // On the other hand, statements in Rust are instructions that perform some action.
    // Statements NEVER return a value, only expressions return (or evaluate to) a value. For
    // example, here `x` is a statement. Here, we are performing the action of binding `x` to the
    // value 5, and not returning any value. That means it is a statement, and statements by
    // themselves return the `()` unit type. Normally that would be fine, but because this
    // function has the return type of `i32`, the Rust compiler expects an expression of type
    // `i32`, not `()` unit.
    // let x = 5;
}

// Here, we have a function called `plus_one` which contains a parameter called `number` with the
// `i32` type. We declare a statement binding the value of the parameter plus 1 to a variable
// called `result`. Since this function does not specify a return type, the return value is
// expected to be a `()` unit.
fn plus_one(number: i32) {
    // So here, the `result` variable returns nothing, and nothing is represented by the `()`
    // unit type, which contains no value.
    // You end statements in semicolons. The last line of a function does not require a semicolon.
    let result = number + 1;

    // You could also just explicitly return `()`, but Rust is already implicitly doing this so
    // that's redundant.
    ()

    // Or even this which ends in a semicolon.
    // ();
}

fn main() {
    let result = plus_one(10000);

    let x = five();
    println!("The value of function five() is: {x}");

    println!("Hello, world!");

    another_function(5, 10.8);

    // Here, `y` is bound to a scope which evaluates to an expression.
    let y = {
        // We define a statement binding 6 to a variable called `x`.
        let x = 6;

        // Next, we make an expression which evaluates to the value of `x` (6) plus 1 which
        // results in 7.
        // Notice here we do not use a semicolon because this is the last line of the scope, aka
        // end-expression. The return value of the scope is 7, which Rust deduces as an `i32`.
        // The binding `y` also deduces to the `i32` type.
        x + 1
    };

    println!("The value of y is: {y}");

    // Here we bind `y` to a scope.
    let y = {
        // This is a statement declaration that binds 500 to the `t` variable, but does not
        // return anything. Statements always evaluate to the `()` unit type and have no value.
        let mut t = 500;

        // This is also considered a statement since it is assigning a value to an existing
        // binding. It is not an expression that evaluates to a value. This also returns the `()`
        // unit type.
        t = 500
    };
}

// You can also define function below `main()` since it's in the same file.
fn another_function(x: i32, y: f64) {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}

fn do_something() {
    let mut x = 5;
    x = 5
}
