## Variables

A variable is an identifier that is bound to a certain value. An **identifier** is the name you write in code, such
as "_x_" or "_guess_". A **value** is a piece of data produced by an expression, such as `5`, `"hi"`, `10 * 2`, etc.

A **binding** is the association between the identifier and a value. For example,

```rust
let x = 5; 
```

This creates a binding that binds the identifier `x` to the value `5`.

If this is confusing, note that a binding just is the precise concept, "variable" is just the term typically used to
describe it; for the most part, they mean essentially the same thing. `x` is both a variable and a binding.

### Mutability

Mutability is a property of the binding that controls whether you can assign (bind) a new value to the identifier, and
whether you get mutable access to the value via that identifier.

By default, all bindings in Rust are **immutable** (meaning unchangeable). Unchangeable in this context means the value
of the variable cannot be changed. Attempting to change the value of a binding will result in a compile-time error.
For example,

```rust
let x = 5;
x = 10;      // The value cannot be changed
```

If a binding's value should be changed in the program, it must be made **mutable** (changeable). This can be done by
annotating the identifier with the `mut` keyword. For example,

```rust
let mut x = 5;
x = 10;     // Will work, the value is mutable
```

Note that the underlying type of a variable will never be mutable. A mutable binding lets you reassign a new value to
the variable. For example,
attempting to assign a numerically-typed variable to a value that is non-numeric (e.g., a string) will result in
compile-time errors even if using `mut`.

```rust
let mut x = 5;
x = "string"    // x cannot be bound to a string
```

### Type Annotation

The type of the variable is inferred by the compiler based on the expression and context. However, you can still
explicitly **annotate** a type if necessary using a `:` suffix. For example,

```rust
let x: u32 = 5;          // Numeric type
let t: & str = "text";   // Textual type
let b: bool = true;      // Boolean type
```

You can also declare a variable without binding it to a value. This can be done only when explicitly type variables
uninitialized. annotating the variable, which leaves it uninitialized.

```rust
let x: u32;
```

### Constants

As mentioned, all bindings default to immutability, however they can be made mutable with `mut`. **Constant**
bindings are always immutable, and attempting to make them mutable with `mut` will result in compile-time errors.

Constants are defined using the `const` keyword, and must always be type annotated. The `let` keyword is not used
with `const`. The naming convention for constant variables is all uppercase words separated by underscores. For
example,

```rust
const SPEED_OF_LIGHT: u32 = 299_792_458;
SPEED_OF_LIGHT = 100_000_000    // const values cannot be changed
const mut z: u32 = 10;          // mut cannot be used with const
```

Unlike normal bindings, constants can be declared at any scope, including globally. This is since constants are often
used by many parts of a codebase, since they are trusted not to change.

The last difference is that constants may only be set to a constant expression. This means they cannot be bound to
the result of a value that could only be computed at runtime.

### Shadowing

Rust allows an identifier name to be reused in the same scope by declaring a new binding with `let`. This does not
require the previous binding to be mutable. For example,

```rust
let x = 10;
let x = 5; // This works
```

This is called **shadowing**. The new binding is the _shadowing binding_, and the earlier one is the _shadowed
binding_.

With `mut`, you change the value of the _same_ binding, while with shadowing you create a _new_ binding, which may
have a different type. For example,

```rust
let mut guess = String::new(); // shadowed binding (textual type)
io::stdin().read_line( & mut guess).unwrap();
let guess: u32 = guess.trim().parse().unwrap(); // shadowing binding (numeric type)
```

Here, the first `guess` is a mutable `String` binding used to store user input from the console. The, we
create a new binding with the same name, but annotated with a different type (a `u32` integer type). The expression
`guess.trim().parse()` converts the **value** of the previous `String` into a new `u32` value, and that new
value is bound to the shadowing binding. The shadowed binding is then rendered inaccessible and all references to
the `guess` identifier are using the shadowing binding.
