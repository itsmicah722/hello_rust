// Constants can be declared globally, and are valid for the entire time the program runs.
const _SPEED_OF_LIGHT: u32 = 299792458; // Meters per second
// let x = 5; `let` cannot be used for global variables

fn main() {
    // A variable is an identifier that is bound to a certain value. The identifier is a name,
    // and the value is a piece of data produced by an expression. The association between the
    // identifier and value is called the "binding".

    // For example, you can bind the identifier (name) `x` to the value `5`. This creates a
    // variable.
    let x = 5;

    // You can also declare a variable without binding it to a value. You can do this by
    // "annotating" the variable, which explicitly states a type to the compiler and to other
    // programmers. For example, a numeric type (e.g., 32-bit unsigned integer). This variable
    // will remain uninitialed until bound to a value.
    let x: u32;

    // Variables in Rust are immutable (unchangeable) by default. This means any attempt at
    // reassigning values to them to new values will result in error; even if the new value is
    // the same as the original variable.
    let z = 17;
    println!("The value of z is: {z}, and is immutable by default");
    // z = 10; this would result in error.
    // z = 17; this would STILL fail even though it's the same value as z.

    // Variables can be made mutable (changeable) via the `mut` keyword. This communicates to the
    // compiler (and other programmers) that this variable is intended to be changed at some
    // point, and resolves immutability errors.
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x was safely changed to: {x}");

    // Constant variables are used with the `const` keyword, and are similar to regular immutable
    // variables in that they are unchangeable. However, constants are ALWAYS unchangeable, and
    // using `mut` will result in errors.
    // - Constants are ALWAYS required to have type annotations on declaration.
    // - Constants are ALWAYS required to use UPPER_CASE naming format.
    // - Constants can be defined at any scope, since they are often used by many different
    // parts of code.
    // - Constants may only be set to a constant expression, not the result of value that could
    // only be computed at runtime.
    const _A: u32 = 15;
    println!("The value of a is: {_A}, and is constant");

    // _A = 15;         Will result in error from reassigning a const
    // let mut _A = 15; Will result in error from trying to use mut on a const

    // Here two variables are being declared with the same name using `let`. This is possible due
    // to the concept of "shadowing" in Rust, which essentially that the newly declared variable
    // will be what the compiler sees when you use the name of the variable.
    // This is useful so that we don't have to use different names for the same variables, (e.g.,
    // y1, y2, etc.)
    // Shadowing is different from making a variable mutable because attempting to reassign the
    // variable to a new value will result in a compile time error when not using the `let`
    // keyword. This is because the variables are still immutable by default, shadowing only is
    // like controlled mutability in a sense.
    let _y = 5;
    let _y = 5 * 6 / 2;
    // _y = 5; will not work since `_y` is still immutable when not using `let` keyword.

    {
        // This is a new scope being defined. We declare a variable y which is assigned to the
        // value of the shadowed `_y` variable. It will be forgotten as soon as the curly
        // brackets end.
        let y = _y * 2;
        println!("The value of _y in the inner scope is {y}");
    }

    // Since the previous curly brackets have closed, what's inside is "out of scope". This
    // means, only the shadowed `_y` variable will be used in this scope.
    println!("The value of y in the outer scope is {_y}");

    // In this case, we have one variable which is textual type, and another of numeric type.
    // Shadowing here allows us to use the same name, instead of different temporary (e.g.,
    // spaces_str, spaces_num, etc.)
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Spaces was shadowed to the number: {spaces}")

    // Note trying to use `mut` here will result in a compile-time error.
    // This is because we're not allowed to mutate a variables type.
    // let mut spaces = "    ";
    // spaces = spaces.len();
}
