<!-- TODO: Make a section or video in the future about expressions in Rust.-->

# Notes

## Types

A **type** is the way Rust interprets a value in memory, and determines what operations can be done on that value.

- There are **built-in** types which are part of Rust.
- There are **user-defined** types which are defined by the programmer.

## Variables

A variable is an item that stores a value in memory. Variables can be declared via the `let` keyword. For example:

```rust
let x = 5; 
```

Variables are **immutable** by default, meaning they cannot be changed. If a variable should be changed, we can
annotate it with the `mut` keyword. For example:

```rust
let mut x = 5; // x is now changable
```

Variables type can be explicitly annotated by using the `:` after the variable name. For example:

```rust
let x: u32 = 5; // sets variable type to unsigned 32-bit integer
```

### Constants

**Constant variables** are used with the `const` keyword, and are similar to regular immutable
variables in that they are unchangeable. However, constants are ALWAYS unchangeable, and
using `mut` will result in errors.

- Constants are ALWAYS required to have type annotations on declaration.
- Constants are ALWAYS required to use UPPER_CASE naming format.
- Constants can be defined at any scope, since they are often used by many different
  parts of code.
- Constants may only be set to a constant expression, not the result of value that could
  only be computed at runtime.

## Functions

**Associated Functions** are part of a user defined type. They are used with the `::` operator. For example, with
the `String` type:

```rust
String::new();
```

## Loops

`loop` keyword can be used to create an infinite loop.

- You can exit an infinite loop via the `break` keyword.
- You can continue to the next iteration of a loop via the `continue` keyword.

## Enumeration

`enum` (_Enumeration_) allows you to create a type by enumerating its possible states; each possible state is known
as a **variant**.

### Result

`Result` is an enum type that represents either a success value, or error value.

- `Ok` contains the success value.
- `Err` contains the error value.

The result type is like any other type, and can have methods defined on it. The `expect()` method is what prints the
error information when something goes wrong.

## Matches

A **match** is made up of _arms_. An arm consists of a _pattern_ to match against, and the code that will be
executed if the arm matches the pattern.

## Shadowing

_Shadowing_ in Rust allows us to create a variable with the same name as a previous variable, and replaces the
original variable's value. This is typically done when we want to convert the type of the original variable.

## Crates

A crate is the smallest unit of code that the Rust compiler will consider at a time. Even when invoking `rustc`
directly, it will consider the source file itself as a crate. Crates can be of two types:

- **Binary**: which means the crate is a standalone executable that is runnable by the operating system. It contains
  a `main()` function entrypoint.
- **Library**: which means the crate does not compile to an executable, but instead contains code which can be used
  by binaries. Library crates do not have `main()` functions.

**SemVer** (_Sematic Versioning_) is a standard for writing version numbers. It is used in the `MAJOR.MINOR.PATCH`
format, and works in Rust this way as well for crate versions.

## Misc

- **Runtime**: The local environment in which a program runs and executes its work, could also mean the total elapsed
  time of the programs' execution.
- **Compile Time**: The period of time when the compiler processes source code into other code that is readily
  executable (or in a library crate just usable).

The `std` keyword in Rust is short for *Standard Library* and contains things you'll use in every codebase.

A lot of stuff in the standard library is already included for you in the **Prelude**, which is a Rust collection of
libraries from `std`.

`fn main {}` is the entrypoint of a Rust program.
