# Borrowing

The issue with using data on the heap is that ownership is moved by default. For example, if a function requires a
`String` argument, ownership of that `String` will be moved into the function body, and then dropped when the
function goes out of scope. To prevent this, you can return the string as a result and ownership will propagate back
to the caller.

If some code just needs to _use_ some value on the heap, and doesn't necessarily need ownership of that value, it's
better to use a reference.

## Reference

A **reference** is a pointer that contains an address of data somewhere in memory. They can be declared using the
`&` keyword. For example,

```rust
let x = 5;
let r = & x;

println!("{r}");
```

`r` is a reference to the value of `x` (5) which is stored on the stack. 

### Ownership

References do **not** own the data they point to, meaning they will not delete or free the data they point to once
out of scope. This feature allows references to use data stored on the heap, without needing to return ownership
back to the original owner. For example,

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

`s1` is bound to a string with the data `Text` that is stored on the heap. `s1` is the owner of that data
on the heap. Next, `result` is bound to the return value of `calculate_length()`, which takes a
reference to a string by reference (because of the `&`). When the function is executed, `str` will exist as a
pointer to the string data on the heap. Once the `str` goes out of scope, it is dropped and nothing happens to the
original string data.

![Reference Ownership Image](../assets/reference_ownership.svg)

### Mutability

References are immutable by default, meaning they cannot modify the data they point to. For example,

```rust
fn main() {
    let str = String::from("hello");
    change(&str);
}

fn change(some_string: &String) {
    some_string.push_str("world"); // Error
}
```

This code results in a compilation error due to mutability of the reference. `some_string` has the `&String` type, 
which is immutable by default. We attempt to add to the string using `push_str` which is considered a modification 
of the original value, which is not allowed. 

To make a reference mutable, you can use `& mut` syntax. For example,

```rust
fn change(some_string: &mut String) {
    some_string.push_str("world");
}
```

This introduces another mutability compilation error. If a reference is mutable, the owner of the actual data the 
reference points to must also be mutable. 

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
> There can be any number of immutable references to a value. However, there can only be **one** mutable 
> reference to a value. 

```rust
let mut var = 10;
let r1 = &mut var;
let r2 = &mut var; // Error

println!("{r1}, {r2}");
```

```rust

```

### Dereferencing

## Borrow Checker
