#![allow(unused)]

// An enum stands for "enumeration", and is a type that contains one or more possible states.
// Each possible "state" is called a variant.
//
// In this case, we have an enum called `IpAddrKind`
// which can hold two different versions of the internet protocol address format. `V4` and `V6`
// are the variants of this enum.
enum IpAddrKind {
    V4,
    V6,
}

// The enum can be used as a struct field that contains information about an ip address.
// Here, the struct holds an address that is stored in a String, and can have one of two variants
// to represent its version; (either 4 or 6).
struct IpAddress {
    kind: IpAddrKind,
    address: String,
}

// A more efficient way is to just store data directly inside the enum variants. Here, each
// variant has an initializer function that can be used to attach tuples, structs, integer types,
// strings, and even other enums.
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Msg {
    content: String,
}

enum OtherMessage {
    Something(String),
}

#[rustfmt::skip]
enum Message {
    Quit,                       // Has no data associated with it at all
    Move { x: i32, y: i32 },    // Has named fields, like a struct
    Write(String),              // Includes a single String
    ChangeColor(i32, i32, i32), // Includes a tuple with 3 integer values
    Something(Msg),             // Includes a single struct
    Other(OtherMessage),        // Includes another enum
}

impl Message {
    fn call(&self) {}
}

fn main() {
    // Each variant of `IpAddrKind` can be made an instance that is assigned to some value.
    // This is done via `::` syntax as the enum variant is separated from the identifier; (i.e.,
    // `IpAddrKind` is the enum identifier, and `V4` or `V6` is the variant that comes after `::`).
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Enum variants can be passed as arguments as well.
    route(IpAddrKind::V4); // Using namespacing
    route(six); // Or as a binding

    // An instance of `IpAddress` is initialized, and `kind` must be either the `V4` or `V6`
    // variant.
    let host = IpAddress {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    // The enum variants can be attached to data directly instead of relying on a struct.
    IpAddr::V4(127, 0, 0, 1);
    IpAddr::V6(String::from("2001:0db8:0000:0000:0000:8a2e:0370:7334"));

    // You can also bind an enum variant (without a value) to a variable.
    let host = IpAddr::V4;
    let ip = host(127, 0, 0, 1); // Now you can use `host` instead of the `IpAddr::V4` namespace.

    // You can call implemented functions of an enum.
    let str = Message::Write(String::from("Text..."));
    str.call();

    // The `Option<T>` enum is extremely useful, created by the standard library and included in
    // the prelude. It contains the variants `None` and `Some(T)` where T can be any type. This
    // solves a problem where if a value can possibly contain nothing, it should be wrapped in an
    // Option.

    // Any type that gets used in place of `<T`> makes the overall `Option<T>` type a different
    // type.
    let some_number = Some(5); // Type: Option<i32>
    let some_char = Some('e'); // Type: Option<char>
    let nothing_number: Option<i32> = None; // Type: Option<i32>

    // The reason some_number and some_char don't need to be type annotated is because Rust can
    // infer their types based on the values passed to Some() variant.
    // nothing_number is bound to None, which doesn't include any data, therefore the type cannot
    // be inferred. Anytime None is used, it's similar to using "null" in other languages, and it
    // essentially means the same thing; there is no valid value.

    // Option<i8> and i8 are NOT the same thing. Rust won't let you pretend like an Option type
    // is a normal type whether it's an integer, string, char, etc. If `sum` was uncommented, it
    // would result in a compilation error since x and y cannot be added.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;

    // You have to convert an Option<T> to whatever T you're trying to perform operations on. The
    // compiler will also force you to handle Option<T> properly to account for if it's None.
}

fn route(kind: IpAddrKind) {
    ()
}
