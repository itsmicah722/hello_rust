## Types

## Unit Types

The **unit** type in Rust is written as `()` that has exactly one value, and that one value is also written as `()`.

The unit type is typically just referred to as _unit_ or _empty value_, etc. All it means is that there is no meaningful
value here. Rust is an expression-based-language, so when an expression does not result in any meaningful value, it
evaluates to `()`; and its type is also `()`. For example,

```rust
let x = 5;
```

Here, we are declaring a _statement_ which binds the value 5 to the variable `x`. Statements in Rust are merely
instructions, and do **not** result in any value. Therefore, `x`

## Scalar Types

## Compound Types

A **type** is the way Rust interprets a value in memory, and determines what operations can be done on that value.

- There are **built-in** types which are part of Rust.
- There are **user-defined** types which are defined by the programmer.

A **scalar** type in Rust represents a single value. Scalars have four main types: integers, floating-points numbers,
booleans, and characters.

- **Integers** are numbers without a fractional component. Integers can either be _signed_ (only positive numbers) or
  _unsigned_ (both positive and negative numbers).
-
