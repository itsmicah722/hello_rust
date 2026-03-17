## If Expressions

An `if` expression allows for certain code to be executed depending on the value of a condition.
`if` expressions can be separated into multiple _branches_ that check for different conditions.
Only one branch can be executed, and each branch must have matching resulting expression types.

### Condition

A **condition** is an expression that must evaluate to a boolean value; either as `true` or `false`.
Conditions must always evaluate to a `bool` value. For example:

```rust
if 10 {}        // i32  (not bool)
if "string" {}  // &str (not bool)
if 'c' {}       // char (not bool)
```

Rust determines the types of these expressions at compile time. Attempting to use any type other
than `bool` in the `if` condition will result in mismatched type compilation errors since Rust.
Conditions are commonly formed with comparison and logical operators, but any expression of type
`bool` is valid.

These "operators" are syntactic sugar for their underlying **overloading methods**. These
overloading methods are functions defined by the Rust standard library that perform operations on
the _operands_ used in an expression.

#### Comparison Operators

**Comparison operators** are operators that compare two values and produce a `bool` result; either
`true` or `false`. They are most commonly used inside conditional expressions.

| Operator | Meaning                  | Overloading Method         |
|----------|--------------------------|----------------------------|
| `==`     | Equal                    | `std::cmp::PartialEq::eq`  |
| `!=`     | Not equal                | `std::cmp::PartialEq::ne`  |
| `<`      | Less than                | `std::cmp::PartialOrd::lt` |
| `>`      | Greater than             | `std::cmp::PartialOrd::gt` |
| `<=`     | Less than or equal to    | `std::cmp::PartialOrd::le` |
| `>=`     | Greater than or equal to | `std::cmp::PartialOrd::ge` |

##### Equal

```rust
let condition = 10 == 10;

// Under the hood:
let condition = std::cmp::PartialEq::eq( & 10, & 10);
```

The `==` operator returns `true` if both values are considered equal. The operator is defined by
the [PartialEq](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/cmp/trait.PartialEq.html "MIT Explanation on PartialEq")
trait.

##### Not Equal

```rust 
let condition = 10 != 5;

// Under the hood:
let condition = std::cmp::PartialEq::ne( & 10, & 5);
```

The `!=` operator returns `true` if the two values are not equal. The operator is defined by
the [PartialEq](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/cmp/trait.PartialEq.html "MIT Explanation on PartialEq")
trait.

##### Less Than

```rust
let condition = 5 < 10;

// Under the hood:
let condition = std::cmp::PartialOrd::lt( & 5, & 10);
```

The `<` operator returns `true` if the left operand is strictly less than the right operand. The
operator is defined by
the [PartialOrd](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/cmp/trait.PartialOrd.html "MIT explanation of PartialOrd")
trait.

##### Greater Than

```rust
let condition = 10 > 5;

// Under the hood:
let condition = std::cmp::PartialOrd::gt( & 10, & 5);
```

The `>` operator returns `true` if the left operand is strictly greater than the right operand. The
operator is defined by
the [PartialOrd](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/cmp/trait.PartialOrd.html "MIT explanation of PartialOrd")
trait.

##### Less Than Or Equal To

```rust
let condition = 5 < = 5;

// Under the hood:
let condition = std::cmp::PartialOrd::le( & 5, & 5);
```

The `<=` operator returns `true` if the left operand is less than or equal to the right operand. The
operator is defined by
the [PartialOrd](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/cmp/trait.PartialOrd.html "MIT explanation of PartialOrd")
trait.

##### Greater Than Or Equal To

```rust
let condition = 10 > = 10;

// Under the hood:
let condition = std::cmp::PartialOrd::ge( & 10, & 10);
```

The `>=` operator returns `true` if the left operand is greater than or equal to the right operand.
The operator is defined by
the [PartialOrd](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/cmp/trait.PartialOrd.html "MIT explanation of PartialOrd")
trait.

#### Logical Operators

**Logical operators** operate on `bool` values and produce a `bool` result. They are used to combine
multiple conditions into a single conditional expression.

| Operator | Meaning     | Overloading Method   |
|----------|-------------|----------------------|
| `&&`     | Logical AND | N/A                  |
| `        |             | `                    | Logical OR                 | N/A                            |
| `!`      | Logical NOT | `std::ops::Not::not` |

##### Logical AND

```rust
let condition = 10 > 5 & & 3 < 7;
```

The `&&` operator returns `true` only if both operands are `true`; if either operand is `false`, the
result is `false.` In the example, the comparison both evaluate to `true`, so that is the final
result.

##### Logical OR

```rust
let condition = 10 < 5 | | 3 < 7;
```

The `||` operator returns `true` if either operand is `true`. In the example, the first comparison
if `false`, but the second comparison is `true`, so the final result is `true`.

##### Logical NOT

```rust
let condition = ! (10 < 5);
```

The `!` operator inverts a boolean value. In the example, the comparison is false, so `!false`
becomes `true`.

### Branches

`if` expressions can have multiple branches that each check for a certain condition.

These branches are not separate from the `if` expression, they are still intrinsic to the same
expression. Only one of these branches will ever be executed during the program, even if multiple
branches have true conditions. For example:

```rust
fn main() {
    if 10 > 9 {
        println!("Ten is greater than nine")
    } else if 5 > 4 {
        println!("Five is greater than four")
    } else {
        println!("None of the above")
    }
}
```

Only the `Ten is greater than nine` text will get printed to the screen since Rust only executes the
first branch with a true condition; all other branches are ignored after a condition is evaluated as
true.

#### If Branch

The `if` keyword declares an if expression which can be used by itself. It is followed by a
condition expression and a block expression, for example,

```rust
fn main() {
    let number = 10;

    if number == 10 {
        println!("The number is 10")
    }
}
```

Here, the condition expression `10 == 10` evaluates to a boolean as `true`, and consequently its
block expression gets executed.

### Else If Branch

The `else if` keyword declares another if expression which must be used directly after a previous
`if` expression. For example:

```rust
fn main() {
    let number = 10;

    if number == 10 {
        println!("The number is 10")
    } else if number == 20 {
        println!("The number is 20")
    }
}
```

### Else Branch

The `else` keyword declares a catch-all expression which must be used after all other branches in
the same block. `else` expressions are only executed if all other conditions in other branches are
false. For example:

```rust
fn main() {
    let number = 10;

    if number == 10 {
        println!("The number is 10")
    } else if number == 20 {
        println!("The number is 20")
    } else {
        println!("The number is neither 10 nor 20")
    }
}
```

### If Expression Rules

All branches in an `if` are treated as a single expression. Each branch of an `if` expression must
return the same type due to the type system. For example:

```rust
fn main() {
    let number = 5;

    let result = if number == 10 {
        "ten"
    } else if number == 5 {
        5 // mismatched type 
    }
}
```

This example will cause compilation errors because the first branch returns a `&str` result, and the
second branch returns a `i32`.

Every `if` expression must be type correct, meaning each branch's block expression results in a
single consistent type. For example:

```rust
fn main() {
    let number = 5;

    if 10 > number {
        10
    } else {
        5
    }
}
```

Here, the `if` returns the consistent type of `i32`, which is type correct.

> [!NOTE]
>
> `if` expressions can also return `()` units, as long as every branch returns `()`.
