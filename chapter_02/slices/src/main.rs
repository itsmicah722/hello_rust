fn main() {
    // A slice is a view into a contiguous sequence of elements within a collection. It allows the
    // use of a portion of data within some collection (e.g., string, array, etc.) using range
    // expressions. For example:
    let array = [1, 2, 3, 4, 5]; // Declare a collection of 5 elements
    let slice: &[i32] = &array[..]; // Declare a slice of that array

    // In this case, we use the

    let s = String::from("This is a sentence.");
    let slice = first_word(&s);
    println!("First word: {slice}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
