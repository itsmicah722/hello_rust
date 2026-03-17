#![allow(unused)]

fn main() {
    // A reference is a value which holds an address (points) to data somewhere in memory.
    // They do NOT own the value they point to, and always point to valid memory. References can
    // point to data both on the stack and the heap. References are declared with the `&` operator,
    // for example:
    let x = 10;
    let reference = &x;

    // Also for heap allocated types
    let s = String::from("Text");
    let reference = &s;

    // NOTE: The act of creating references is known as "borrowing" in Rust. In this case, `&x` is
    // 'borrowing' `x`, and `&s` is 'borrowing' s. References must follow strict rules of ownership
    // and mutability, enforced by the "borrow checker". The term "borrow" and "reference" are
    // interchangeable.

    // A reference's scope is known as its "lifetime", and is not identical to the lexical (block
    // based) scope of the binding that it points to. Rust references follow "Non-Lexical
    // Lifetimes" (NLL), and the lifetime is based on usage, not lexical scope. For example:
    let mut x = 5;
    let r = &x;

    println!("{r}");

    // Unlike `x`, `r` does not end at the end of this current block in the main function; it
    // actually ends at the `println!()`. This is because a reference's lifetime ends at the last
    // point it is used. If we didn't print the value of `r`, its lifetime would end immediately
    // since it is never used.

    // This plays into the rules of mutability that all references must follow. Similar to
    // regular bindings, references are immutable by default.
    let x = 10;
    let r = &x;
    // x = 100; Error: Cannot assign twice to immutable variable `x`

    // When mutating the value of a reference, the original owner's value will be changed, not
    // the reference variable itself. We do this using the `mut` keyword. For example:
    let mut c = 'c'; // Original owner must use `mut` as well
    let r = &mut c;

    // In Rust, mutability for all references follows a very strict rule:
    // At any given time you can have either: ONE mutable reference (OR) or ANY number of immutable
    // references. For example:
    let mut s = String::from("Text");
    let r1 = &mut s; // Last used here -> lifetime ends
    let r2 = &s; // Last used here -> lifetime ends
    let r3 = &s; // Last used here -> lifetime ends

    // There's also a similar rule for improperly combining mutable and immutable references. For
    // example:
    let mut s = String::from("Hello");

    let r1 = &s; // Lifetime starts
    let r2 = &s; // Lifetime starts
    println!("{r1}, {r2}"); // `r1` and `r2` last used here -> lifetime ends

    let r3 = &mut s; // Lifetime starts
    println!("{r3}"); // `r3` last used here -> lifetime ends

    // This compiles since `r1` and `r2` are both immutable, and the rule states that any number
    // of immutable references can exist in the same lifetime. Additionally, `r3` is mutable, but
    // since the previous immutable reference's lifetime had already ended, the borrow checker
    // allows it.

    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // ERROR HERE

    println!("{r1}, {r2}, {r3}");

    // This would not compile since a mutable reference is declared in the same lifetime as
    // other immutable references. The lifetimes for `r1`, `r2`, and `r3` all end at the same
    // point, thus cannot exist together unless either the mutable reference is removed or the
    // other immutable references are removed.

    // The term "dangling" means something points to memory that is not valid. This term can
    // apply to both pointers and references, however references are typically used more in Rust
    // due to safety. References must ALWAYS point to valid memory. For example:
    let r: String;

    {
        let str = String::from("Text");
        let r = &str;
        println!("Inside the inner scope: {r}");
    }

    // println!("Outside inner scope: {r}");

    // If the outer println! was not a comment, the code would not compile since the memory
    // owned by `str` has been freed; thus, making any reference to that memory considered
    // "dangling".
}
