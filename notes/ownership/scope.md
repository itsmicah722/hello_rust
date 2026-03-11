## Scope

A **scope** is the region of code where an item is valid and can be used. If an item is out of scope, it is not valid and cannot be used. An item will always remain in scope until it goes *out-of-scope*.

## Variable Scope

For a local binding created with `let`, its scope usually begins at its declaration and continues to the end of the enclosing block.

```rust
// s is not in scope here
{
    // s is not in scope yet

    let s = "string"; // s enters scope here

    // s is in scope for the remainder of the block
}
// s goes out of scope here and is no longer valid
```

This block expression contains a scope; code before or after the block is not in scope. In this case, the binding `s` is declared and assigned to a string literal and has a scope:

- Before the block, `s` does not exist yet; it is out of scope.
- After the block, `s` goes out of scope and can no longer be used.
- Inside the block, but (before) its declaration, `s` is not in scope yet.
- Inside the block, `s` is in scope from its declaration, to the end of the surrounding block.

A **block** creates the region where variables may live, but each variable only starts existing form its own declaration onward. Variables are not in scope before their block, before their declaration, or after their block ends.

```rust
{
    let x = 1;

    // x is in scope here, s is not

    let s = "hello";
    
    // x is in scope here
    // s is also in scope here
}
// All bindings inside the block go out of scope here
```

### Inner Block

Blocks can include their own inner blocks, which can also include their own inner blocks, and so on. An **inner** block has access to all bindings in all surrounding outer blocks. However, an **outer** block does not have access to any bindings in its contained inner blocks. For example,

```rust
{
    let x = 10;

    {
        let y = "text";
        println!("{x}"); // Inner block has access to x
        println!("{y}");
    }
    
    println!("{x}");
    println!("{y}"); // Error: Cannot find value `y` in this scope
}
```

In this example, two blocks are defined. The outer block declares a binding `x` assigned to a number. The inner block declares a binding `y` assigned to a string. In this case, the inner block can use the outer binding `x`, but the outer block cannot use the inner binding `y`.

The same rule applies to other constructs as well, such as `if` blocks. For example,

```rust
let number = 5;

if number > 3 {
    let message = "big";
    println!("{message}");
}

println!("{message}"); // Error: Cannot find value `message` in this scope
```

Here, `number` is in scope for the whole surrounding block after its declaration (including the if block). However, `message` only exists in scope within the `if` block. Attempting to use `message` outside of its scope will result in compilation errors.