# Move Semantics

Types that implement the `Copy` trait will never be moved. If a type implements the `Drop` trait cannot also 
implement the `Copy` trait.

Types that do not implement the `Copy` trait will always be moved, even if copying is cheap.

Types that are moved will transfer ownership to functions if used. For example,

```rust
fn take_ownership(str: String) {
    println!("{s}");
}

fn main() {
    let s = String::from("Text");
    
    take_ownership(s);
}
```

In this case, the ownership of the string value of `s` to the `take_ownership()`. The `str` parameter now owns the 
string data on the heap, and after the function exits
