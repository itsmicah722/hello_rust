# Loops

A **loop** is an expression used to indefinitely execute instructions within its scope. Loop's contain a **loop body**, which contains the code being executed, and behaves similarly to a typical block expression. There are different kinds of loops represented by different keywords: `loop`, `while`, and `for`.

## Infinite Loop

The `loop` keyword can be used to declare a loop which does not exit and indefinitely executes code inside its body.

```rust
loop {
    println!("again")
}
```

This loop will continue printing `again` to the screen forever.

### Never Type

Control flow never reaches the end of an infinite loop, so the expression never produces a value. The [!](types.md#never-type) (*Never Type*) is a real type that has no possible values at all. For example,

```rust
let result = loop {
    5
}
```

The `result` binding will have type `!` and cannot hold **any** possible value at all. Control flow will also never reach any code defined after this loop either.

### Break

If a loop is intended to return an expression or exit at some point, the `break` keyword must be used. `break` exits the loop, and can optionally specify an expression to return from the loop. For example,

```rust
let mut count = 0;

let result = loop {
    println!("Count: {count}");

    if count == 10 {
        break count;
    }

    count += 1;
}
```

This declares a mutable binding called `count`, which increases by 1 on every iteration of the loop. We bind `result` to the value returned from the loop. To do this, an if expression is used to execute once `count` reaches the value 10, in which case `break` is used to exit the loop and return an expression. The value of the expression in this case would be 10, as an `i32`, which would be assigned to `result`.

> [!NOTE]
>
> If `break` is not used, the loop expression will always be of type `!` and never exit or return a value.

### Continue

The `continue` keyword is used to stop the current iteration of a loop and immediately start the next one. `continue` cannot be used to return an expression since it does not exit the loop. For example, 


```rust
let mut count = 0;

let result = loop {
    println!("Count: {count}");

    if count == 5 {
        continue; // skipped
    }

    if count == 10 {
        break count;
    }

    count += 1;
}
```

This loop will skip an iteration of the loop once `count` reaches the value 5. Control flow will continue to the 6th iteration of the loop, but skip the 5th.

## While Loop

The `while` keyword can be used to declare a conditional loop. Similar to `if`, the condition specified must evaluate to a `bool`, and will only trigger execution if it is `true`. For example,

```rust
let count_down = 10;

while count_down != 0 {
    println!("{number}!");

    count_down -= 1;
}

println!("LIFTOFF!!!");
```

This loop will continue executing as long as `count_down != 0` evaluates to `true`. Once the condition is false (the count down reaches zero), the loop will stop executing. This is often cleaner than a `loop` if the goal is to keep executing according to some condition. A `loop` would force you to specify some sort of `if` expression that checks for a value, and then use `break` or `continue` if said value is the case.

You could also use a `while` expression to loop over elements of a collection such as an array. For example,

```rust
let array = [10, 20, 30, 40, 50];
let mut index = 0;

while index < 5 {
    println!("The value is: {}", array[index])

    index += 1;
}
```

This program declares an array with 5 elements and a mutable index which increases on every iteration of the loop. On each iteration, the index corresponds to a different element in the array, and eventually iterates through every element. The value `index` will eventually reach 5, which is the length of the array, but the expression `index < 5` will evaluate to `false` and stop execution of the loop.

This works, but is error-prone since if the length of the array or initial value of index is changed, the condition must also be changed. If the condition is not correct, the program will crash with an index out of bounds error.

## For Loop

The `for` keyword can be used to declare a loop that iterates elements in a collection. The code in the loop's body will execute for each iteration. For example,

```rust
let array = [10, 20, 30, 40, 50];

for element in a {
    println!("The value is: {element"});
}
```

A `for` loop increased the safety of the code and eliminated the chance of index out of bounds errors at runtime. This is because a condition doesn't need to be manually specified for when to end the loop.

### Range Expressions

`for` loops can utilize [range expressions](ranges.md) which create a span of values between two bounds. For example,

```rust
let range = 0..=10;

for count_down in range.rev() {
    println!("{count_down}!");
}

println!("LIFTOFF!!");
```

This declares a [inclusive range](ranges.md#inclusive-range) expression which includes both the lower bound `0` and the upper bound `10`. We iterate through the range and use the `rev()` function to reverse the values inside the range. This effectively prints a countdown message from `10` to `0` and then lift off!