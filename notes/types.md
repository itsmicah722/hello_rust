<!-- TODO: Finish this section -->

## Types

## Unit Type

The `()` type is called the **unit** type and has exactly one value: `()`. The unit type indicates that there is no meaningful value associated with an expression. This doesn't mean there is *no* value, it does have a value which is represented with unit.

Think of an expression like a box which holds paper (a value). If a box doesn't have any paper in it, the box still exists, it's just called an empty box. The rule is there cannot be no box, there must always be a box, even if it's empty. In Rust, an empty box is called "unit" with its own special type: `()` and special value: `()`. For example,

```rust
let x: () = ();
```

Here, `x` is type annotated with unit and bound to the value unit. 

```rust
fn do_nothing() {}
```

In Rust, functions that do not specify a return type implicitly expect unit. This is the same as:

```rust
fn do_nothing() -> () {
    ()
}
```

If a block expression ends in a statement, the result of the block expression is unit. For example,

```rust
fn main() {
    let result = {
        let x = 5;
        x + 10;
        // ()   unit is returned here 
    }
}
```

The block expression ends in a statement due to the `;`, which results in unit. The type of result will be `()` and its value is also `()`.

> [!NOTE]
>
> The expression `x + 10` is still an `i32`, but because it's made a statement from the `;`, its value is discarded. After the value is discarded, there is no expression left to return from the block, so `()` is the result of that block.

This works the same for `if` expressions. For example,

```rust
let result = if 5 < 2 {
    println!("2 is greater than 5");
    2;
} else if 5 > 2 {
    println!("5 is greater than 2");
    5;
}
```

Both branches end with expressions that were made statements with `;`. This means the branch expressions are discarded and `()` is returned. The `if` expression now returns unit, and `result` has the type and value of unit.

## Never Type

## Scalar Types

## Compound Types