# Structs

A _struct_ is a **user-defined** data type that groups related values together into a single type.
Each value of a struct is stored by corresponding _field_, where each field has its own type.

```rust
struct Human {
    name: &str,
    age: u8,
}
```

Here, we have declared a struct with the type `Human`, that has two fields:

- `name`: Can store a string slice.
- `age`: Can store an unsigned 8-bit integer.

In this case, `Human` is just a type, just like `i32`, `bool`, `String`, etc. The difference is
the type is defined by you.

> [!NOTE]
>
> A struct is like a blueprint that defines what data a value must contain. It does not create
> anything on its own.

> [!IMPORTANT]
>
> The layout of a struct is fixed. This means, after a struct has been declared, fields cannot
> be added or omitted, and the field types cannot be changed.

## Instance

An _instance_ of a struct is a **concrete value** of that struct type. It bundles related data
into a single value that can be bound to a variable, passed around, borrowed, and if mutable,
modified.

To create an instance, you provide concrete values for each of its fields using `field: value`
syntax:

```rust
struct Cat {
    name: String,
}

fn main() {
    let kitten = Cat {
        name: String::from("kitty"),
    };
}
```

Here, `Cat { name: String::from("kitty") };` creates an instance of type `Cat`, and `kitten` is
the binding that owns that value. The `name` field is initialized with a string value.

## Implementation

The `impl` keyword is used to define an _implementation block_, used to implement types.
In the context of a struct, an `impl` block would look like this:

```rust
struct Dog {
    name: String,
    age: u8,
}

impl Dog {
    fn name(&self) {
        println!("The dog's name is {}", self.name)
    }
}
```

In this case, `Dog` is a type that the `impl` block implements functionality for. You can define
multiple implementations for a single type.

```rust
struct Dog {
    name: String,
    age: u8,
}

impl Dog {
    fn name(&self) {
        println!("The dog's name is {}", self.name)
    }
}

// Another implementation
impl Dog {
    fn age(&self) {
        println!("The dog is {} years old", self.age);
    }
}
```

In this case, defining an additional implementation is redundant and unnecessary, but according
to the book, this feature will be useful later on.

## Associated Functions

An _associated function_ is a function defined inside an `impl` block that belongs to a type, but
does **not** operate on a specific instance of that type.

```rust
struct Dog {
    name: String,
    age: u8,
}

impl Dog {
    fn new(name: &str) {
        println!("The dog's name is {name}")
    }
}
```

Here, `new()` is a function associated with the `Dog` type, but it doesn't operate on any
instance of that type. Since associated functions don't have access to any instances, they must use
a different call syntax:

```rust
Dog::new(String::from("Mochi"));
```

The struct name `Dog` uses `::`, followed by the name of the associated function `new()` instead
of object-oriented dot notation syntax.

## Method

A _method_ is an associated function of a type that has `self` as its first parameter, and operates
on an instance of that type.

```rust
struct Dog {
    name: String,
    age: u8,
}

impl Dog {
    fn age(self) {
        println!("The dog's name is {name}", self.name)
    }
}
```

Here, `age()` is a method implemented for the `Dog` struct that prints the value of the `age`
field for a particular instance. The instance that the method receives depends on the caller.
For example,

```rust
fn main() {
    let pup = Dog {
        name: "Bob",
        age: 3,
    }

    pup.age();
}
```

The `pup` instance initializes the `age` field as the value 3. To call the method, we use `.` dot
notation syntax, where the instance name uses a `.`, followed by the method name. When the `age()`
method is called, it receives the `pup` instance by value and prints the age field for that
instance.

## Usage Of Self

Inside a struct's implementation block, there four primary methods to accessing internal struct
data in relation to some instance.

| Type        | Usage                                               |
|-------------|-----------------------------------------------------|
| `Self`      | When referring to the type itself.                  |
| `self`      | When the method should own the instance.            |   
| `mut self`  | When the method should own and change the instance. |   
| `&self`     | When the method should borrow the instance.         |   
| `&mut self` | When the method should borrow and change instance.  |   

### Self

`Self` is an alias for a struct type, and is used inside an `impl` block for that struct.

```rust
struct Dog {
    name: String,
}

impl Dog {
    fn new(name: String) -> Self {
        Self { name }
    }
}
```

Here, `Self` is an alias for `Dog`. It's the same as rewriting the function as:

```rust
fn new(name: String) -> Dog {
    Dog { name }
}
```

Using `Self` is more idiomatic than repeating the struct name, and becomes especially useful in
generic code.

### self

`self` is an **immutable** instance received _by value_ and takes ownership locally within a method.
If a struct type implements the `Copy` trait, `self` will receive a copy of the instance, and
won't take ownership.

```rust
struct Dog {
    name: String,
}

impl Dog {
    fn print_name(self) {
        println!("The dog's name is {}", self.name)
    }
}

fn main() {
    let pup = Dog {
        name: String::from("Charlie"),
    };

    dog.print_name();
    dog.print_name(); // Error
}
```

Here, the `print_name()` method signature specifies `self` as a parameter. This means the method
will receive the `pup` instance in by value, and also take ownership of `pup` and move it into the
function scope. Because `pup` is moved and ownership is not returned, any attempt to use `pup` after
calling `print_name()` will result in a compilation error.

### mut self

`mut self` is a **mutable** instance received _by value_ that takes ownership locally within a
method. As mentioned, if the struct type implements the `Copy` trait, `self` will receive a copy of
the instance, and won't take ownership.

```rust
struct Dog {
    age: u8,
}

impl Dog {
    fn grow(mut self) {
        self.age += 1;
        println!("Happy {age} birthday")
    }
}

fn main() {
    let pup = Dog { age: 3 };
    dog.grow();
}
```

Here, the `grow()` method signature specifies `mut self` as a parameter. This means it receive a
mutable instance of the struct. This means it will receive the `pup` instance by value, take
ownership, and be allowed to change the instance. Changing the instance here means mutating the
values of any fields associated with the instance, (i.e., `name` or `age`). As mentioned before
if the struct implements the `Copy` trait, the method will receive a copy of the instance
instead of receiving ownership.

### &self

`&self` is an immutable borrow of the instance. This allows the method to read from the instance
without taking ownership of it.

```rust
struct Dog {
    name: String,
    age: u8,
}

impl Dog {
    fn bark(&self) {
        println!("{} has barked!", self.name);
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Buddy"),
        age: 3,
    };

    dog.bark();
    dog.bark(); // No error
}
```

The `bark()` method signature specifies `&self` as a parameter. So, the method will receive an
immutable reference to the struct instance, without taking ownership. The benefit of using a
borrow instead of taking ownership is the method does not need to explicitly return the instance
back to the caller. `bark()` can be called multiple times without compilation errors.

### &mut self

`&mut self` is a mutable borrow of the instance. This allows the method to read and changed
without taking ownership of the instance.

```rust
struct Dog {
    name: String,
}

impl Dog {
    fn rename(&mut self, name: String) {
        self.name = name;
        println!("The dog has been renamed to {}", self.name);
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Cupcake"),
    };

    dog.rename("Millie");
}
```

Here, the `rename()` method signature specifies a `&mut self` parameter. So, it will receive a
mutable reference to the instance without taking ownership. Additionally, values associated with
the fields of the instance can be changed. mentioned, this is a borrow so ownership does not need to
be returned.
