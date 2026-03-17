# Expressions

An **expression** is a piece of code which results in some value. The value of an expression will
also have a type which matches the value. this will be most of the code written. For example:

```rust
5
```

`5` is a value; therefore, `5` itself is an expression by definition, with the type `i32`. This is
because as mentioned, all expressions result in some value.

```rust
100 + 200 * 2 / 2
```

This is a mathematical expression which results in the value `300`. Again, since `300` is a value,
this is also considered an expression.

### Block Expressions

**Block** are created with the `{ ... }` syntax, and are also considered expressions (a.k.a. _block
expressions_). Block expressions can have two outcomes:

- The last line of the block expression does **NOT** have a `;` semicolon at the end: This indicates
  that the last line is an expression, which means the block expression returns a value. - The last
  line of the block expression **DOES** have a `;` semicolon at the end: This indicates that the
  last line is a statement, which means the block expression returns [unit](types.md#unit-type) (no
  meaningful value).

For example:

```rust
{
let x = 5;
x + 1
}
```

This defines a block expression that contains a variable `x` bound to the value 5. Next, the
expression `x + 1` is declared (without) a semicolon. This indicates that `x + 1` is an expression;
and since it is the last line of the block expression, it will be the returned value of that block
expression. The result of this block expression is the value: 6.

```rust
{
let x = 5;
x + 1;
}
```

This defines the same block expression as before, but the last line does not contain a semicolon.
This indicates that `x + 1;` is a statement, and not an expression. This means the returned value of
the block expression is `()`.

```rust
let outer_x = {
let inner_x = 5;
inner_x + 1
}
```

This defines the same block expression, but binds its returned value to a variable `outer_x`. Since
the last line in the block expression does not have a semicolon, it is considered an expression. The
value of that expression (`6`) will be bound to the variable assigned `outer_x`. The variable
`outer_x` will also have the same type as the returned value.

```rust
let outer_x = {
let inner_x = 5;
inner_x + 1;
}
```

This defines the same block expression, but the last line does have a semicolon. This means it is
considered a statement, and the value of that statement is unit. This means `outer_x` will be bound
to unit and have the type of unit.

> [!NOTE]
>
> Block expressions **must** return values of the same type as variables bound to them. For example,
> if `outer_x` is type annotated as an `char`, but the block expression returns a numeric typed
> value like `6`, there will be compile-time errors due to mismatched types.

[Function bodies](functions.md#body) are also considered block expressions, and the same
principles apply. For example:

```rust
fn double(x: i32) -> i32 {
    x * 2
}
```

This function specifies a [return type](functions.md#return-type) of `i32`. This means the block
expression (`{ ... }
` function body) will expect to result in an expression which evaluates to a value of type `i32`.
Hence, no semicolon
for the last line of the block expression.

```rust
fn do_something() {
    println!("do something!");
    5;
}
```

This function does not specify a return type, which means Rust implicitly treats the function as
return unit `()`. Even though `5` by itself is an expression, we must use a semicolon to indicate
that it is a statement. If we did not use a semicolon, `5` would become an expression, which would
result in compile-time errors since the function expects unit; not an `i32` (which is a numeric
type, and not a unit type).

```rust
fn do_nothing() -> () {
    5;
    ()
}
```

You could also explicitly specify the return type as unit and literally return unit, but that's
redundant.

## Statements

A statement is an instruction that performs an action and does **not** return a value. Statements
_can_ be bound to expressions, and _can_ perform actions involving expressions; but statements
themselves are not value-producing expressions. For example:

```rust
let x = 5;
```

This introduces an identifier called `x`, and binds the value `5` to that identifier. Even though
`x` is used to store the expression `5`, the construct itself of `let x = 5;` is **not** an
expression you can use as a value.

```rust
let a = ( let b = 5); // Not allowed
```

This is illegal since statements cannot be assigned to other statements. `let b = 5` isn't an
expression we can bind an identifier to, resulting in compile-time.

> [!TIP]
>
> let-statements are pieces of syntax the language executes, and do not have types or values.
> Bindings do in fact have types and values. Statements just instructions, a variable stores an
> expression assigned to it.

### Semicolons

`let` statements must always include a semicolon at the end of the line. Not doing so will result in
compile-time errors. However, as discussed previously in [block expressions](#block-expressions),
semicolon placement matters when at the last line of a block expression.

Adding a semicolon at the last line of a block expression simply discards the resulting value. This
is fine if the function does not have a return type. For example:

```rust
fn do_nothing() {
    5;
}
```

Even though `5` is an expression with the type `i32`, we add a semicolon after the expression which
discards its value. This means the returned value of this function becomes `()`, which agrees with
what Rust implicitly expects from functions with no return type specified.

```rust
fn return_num() -> i32 {
    5; // Error
}
```

Here, discarding the expression with a semicolon will result in a `()` unit; which will cause a
compilation error since the function expects an `i32` typed return value, not a unit.

```rust
fn return_num() -> i32 {
    let mut x = 5;
    x = 1 // Error
}
```

At the end of block expressions, assignments will always result in `()`. Again, this will result in
error if a return type is specified for the block expression.
