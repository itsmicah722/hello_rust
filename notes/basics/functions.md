# Functions

A **function** is a named chunk of code that runs code when called. It can optionally take inputs called _parameters_, and it may give you an output called a _return value_. For example,

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let x = add(2, 3);
    println!("{x}");
}
```

### Function Call

To **call** a function means to run a function at that line. In `main`, the code `add(2, 3)` is a function call.

### Caller

The **caller** is the code (or function) that makes the function call. Here, `main` is the caller of `add`, because the
main function contains the function call.

### Pass

To **pass** a value means to send or give this value to the function call so the function can use it. Here, we _pass_
`2` and `3` to add.

### Argument

An **argument** is the value itself that you pass in a function call if there are function inputs. In `add(2, 3)`, the
arguments are `2` and `3`. These values will be used in the function's execution when it runs.

### Parameter

A **parameter** is the named placeholder variable in a function that receives an argument. Parameters are
effectively inputs of the function, and are optional. In `fn add(a: i32, b: i32) -> i32`, the parameters are `a` and
`b`.

> [!WARNING]
>
> The terms _argument_ and _parameter_ may be thrown around interchangeably, but do **NOT** mean the same thing.

## Signature

A function's **signature** describes its shape, including the name, parameters, and return type. However, the
function signature does not include the body. For example,

```rust
fn add(a: i32, b: i32) -> i32
```

> [!NOTE]
>
> Parameter types are included in the signature. Parameter names don't change the signature.

## Body

The function **body** includes the `{ ... }` block, and is the code which is executed when the function is called.
For example,

```rust
fn add(a: i32, b: i32) -> i32 {
    // Body starts
    let sum = a + b;
    sum
    // Body ends
}
```

A function body is a [block expression](eop.md#block-expressions), so it can produce a value.

## Return Type

The function **return type** is the type after written the `->`. It can be any type, for example,

```rust
fn add(a: i32, b: i32) -> i32 { ... }
//                        ^^^ return type
```

If the return type is omitted, Rust implicitly treats the return type as `()` [unit](types.md#unit-types). For example,

```rust
fn do_something() {
    println!("Do something");
}

// Explicitly using -> () here does the same thing as
// not specifiying a return type at all
fn do_something_explicit_unit() -> () {
    println!("Do something");
}
```

Return types must always match the value a function returns. Because blocks are expressions, Rust returns the value of
the **final** expression in a body (the one without a `; `) _as long as it matches the return type._ For example,

```rust
fn double(x: i32) -> i32 {
    x * 2   // no semicolon => this expression is the returned value
}
```

Adding a semicolon turns an expression into an _expression-statement_, which will result in `()`. This cause
compilation errors if a function expects a certain return type, and the returned value is unit. For example,

```rust
fn double(x: i32) -> i32 {
    x * 2;  // semicolon => returns (), not i32 => type mismatch
}
```
