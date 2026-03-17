# References

A **reference** is a pointer that contains an address of data somewhere in memory. They can be
declared using the `&` keyword. For example:

```rust
let x = 5;
let r = & x;

println!("{r}");
```

Here, `r` is a reference that contains the address in memory of `x`. References always contain the
_values_ of bindings, so printing `r` will output 5.

The act of creating a reference is called _borrowing_. The reference itself is a _borrow_ or
_borrowed reference_ of the value it points to. For example:

```rust
let s = "...";
let r = & string;
```

Here, `s` owns the data, and `r` is a reference to `s`, thus `s` has been borrowed by the
reference. The [borrow checker](https://rustc-dev-guide.rust-lang.org/borrow-check.html) is the
enforcer of an exhaustive list of rules Rust explicitly checks for to remove the possibility of
memory safety problems.

## Ownership

References do **not** own the data they point to, meaning they will not delete or free the data they
point to once out of scope. This feature allows references to use data stored on the heap, without
needing to return ownership back to the original owner. For example:

```rust
fn main() {
    let s1 = String::from("hello");
    let result = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(str: &String) -> usize {
    str.len()
}
```

`s1` is bound to a string with the data `Text` that is stored on the heap. `s1` is the owner of that
data on the heap. Next, `result` is bound to the return value of `calculate_length()`, which takes a
reference to a string by reference (because of the `&`). When the function is executed, `str` will
exist as a pointer to the string data on the heap. Once the `str` goes out of scope, it is dropped
and nothing happens to the original string
data. ![Reference Ownership Image](../assets/reference_ownership.svg)

## Mutability

References are immutable by default, meaning they cannot modify the data they point to. For example:

```rust
fn main() {
    let str = String::from("hello");
    change(&str);
}

fn change(some_string: &String) {
    some_string.push_str("world"); // Error
}
```

This code results in a compilation error due to mutability of the reference. `some_string` has the
`&String` type, which is immutable by default. We attempt to add to the string using `push_str`
which is considered a modification of the original value, which is not allowed.

To make a reference mutable, you can use `& mut` syntax. For example:

```rust
fn change(some_string: &mut String) {
    some_string.push_str("world");
}
```

This introduces another mutability compilation error. If a reference is mutable, the owner of the
actual data the reference points to must also be mutable.

```rust
fn main() {
    let mut str = String::from("hello");
    //      ^^^ here
    change(&mut str);
    //         ^^^^ here
}

fn change(some_string: &mut String) {
    //                     ^^^^ here
    some_string.push_str("world");
}
```

For the reference to be properly mutable, `mut` must be specified:

- In the `let` declaration of the original owner binding
- In the call arguments
- In the function signature's parameter list

> [!IMPORTANT]
>
> There can be any number of immutable references to a value. However, there can only be **one**
> mutable
> reference to a value in a given scope.

```rust
let mut var = 10;
let r1 = & mut var;
let r2 = & mut var; // Error

println!("{r1}, {r2}");
```

Several big problems can happen if multiple references mutate the same value; the biggest
being [data races](https://doc.rust-lang.org/nomicon/races.html). Multiple immutable references are
fine because every reference is seeing the same value.

```rust
fn main() {
    let mut s = String::from("Text");

    {
        let r1 = &mut s;
        s.push_str(" more text...");
    }

    let r2 = &mut s;
    s.push_str(" even more text...")
}
```

This works because the references are declared and mutate values in separate scopes. `r1` will
mutate `s`, then get dropped before `r2` is declared in the outer scope.

```rust
fn main() {
    let mut s = String::from("Text");

    let r1 = &s; // No problem
    let r1 = &s; // No problem
    let r1 = &mut s; // BIG PROBLEM
}
```

Users of immutable references don't expect the value they're referencing to suddenly change. This is
because references' scope start when they're declared, and end at the last point of which they're
used. For example:

```rust
fn main() {
    let mut s = String::from("Text");

    let r1 = &s; // No problem
    let r2 = &s; // No problem
    println!("{r1}, {r2}");
    // References r1 and r2 will not be used after this point

    let r3 = &mut s; // No longer a problem
    println!("{r3}")
}
```

The scopes of `r1` and `r2` end at `println!()` since that is the last time they are used. The next
reference is mutable, but does not trigger a compilation error because it's declared after the scope
of previous immutable references. As long as the scopes don't overlap, one mutable reference and
multiple immutable references can be used.

## Dangling References

A **dangling reference** is a reference that points to memory that is no longer valid. The original
owner that the reference points to may have been cleaned up, dropped, or gone out of scope. It's
like an address to a house that's been demolished. For example:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
```

The `dangle` function returns a reference to a `String`. It declares a new string `s` and assigns it
to allocated data on the heap. The function returns the `&s` expression which references that
string. Once `s` goes out of scope, the returned reference will not point to valid memory, thus
making it a _dangling_ reference. This will result in a compilation error, since the program tries
to borrow a value that does not exist.

The solution would be to return `String` to provide the caller with ownership.

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```
