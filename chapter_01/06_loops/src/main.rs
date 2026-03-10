#![allow(unused)]

fn main() {
    // The `loop` keyword can be used to create an infinite loop. A loop is an expression that
    // repeats execution of its body continuously (body is just the block expression).
    loop {
        // Loop start
        println!("again!");
        break;
        // Loop end
    }

    // The `break` keyword can be used to exit the current loop iteration and stop the loop. It
    // can also be used to return the value of an expression.
    // The `continue` keyword can be used to skip the current iteration of the loop, and move on
    // to the next iteration of the loop. For example:
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 5 {
            println!("Skipping...");

            // Skips the 5th iteration of the loop.
            continue;
        }

        if counter == 10 {
            println!("Breaking...");

            // Exits the loop and returns the value of counter as the result of the block
            // expression. This value gets assigned to `result`.
            break counter;
        }
    };

    // A loop without a break expression is considered "diverging" and has type `!`, which is
    // called the "never" type. Also, break and continue expressions themeselves
    // let never = never_come_back();

    // It is possible to have loops within loops, `break` and `continue` apply to the innermost
    // loop at that point. You can optionally use labels to represent the different
    // loops with `'label: ` syntax. Now, `break` or `continue` can be used on any loop with a
    // specified label instead of only the innermost loop.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            } else if count == 2 {
                // Break the `counting_up` outer loop, not the current innermost loop.
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // The `while` keyword can be used to create a conditional loop, which only executes when an
    // expression in the loop evaluates to true. This is useful when a loop should not be
    // infinite and must stop when a certain condition stops being the case.
    let mut number = 3;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // This is essentially the same as writing an if expression which checks the value of count_down
    // and using `break` to stop the loop when one of the branch conditions is true. A `while`
    // loop is just a cleaner and less redundant method to the same result.

    // The `for` keyword can be used to create an iterative loop to execute some code for each
    // item in a collection.
    let array = [10, 20, 30, 40, 50];
    for element in array {
        println!("The value is: {element}")
    }

    // Creates a half-open range that includes both the upper and lower bounds. In the `for` loop,
    // `rev()` is used to reverse the values in the range. Each value is printed to the screen,
    // effectively counting down from 10 to 0 and then lift off!
    let range = 0..=10;
    for count_down in range.rev() {
        println!("{count_down}!");
    }
    println!("LIFTOFF!!");
}
