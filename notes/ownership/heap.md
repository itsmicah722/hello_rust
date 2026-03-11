# Heap

## Accessing The Heap

Data on the heap is not accessible to a program without metadata describing where it is in memory. For example,

- `String`
- `Vec<T>`
- `Box<T>`
- `HashMap`

The actual data for these live on the heap, because the size is not known at compile time. The **metadata** that describes the heap data still lives on the stack. For instance, a `String` is actually three values stored on the stack: a `pointer`, `length`, and `capacity`.

```rust
let s1 = String::from("hello");
```

Visually, this looks like the following:

![Heap Metadata Image](../!assets/heap_metadata.svg)

The `s1` binding is **not** the string itself, it's a construct that holds information about the string's data on the heap. The actual text `hello` is the data that's stored on the heap. 

### Pointer

A pointer is an address of some location in memory. The address is not the data itself, it just tells you where to find it. For example, in a `String`, the pointer points to the first byte of the string data on the heap.

> [!IMPORTANT]
>
> In normal Rust code, you usually do **not** work with raw pointers directly. Rust handles that safely for you.

### Length

The **length** indicates how much of the allocated space is currently filled with actual data. For example, `"hello"` would have a length of `5`, because there are 5 bytes being used.

### Capacity

The **capacity** indicates how much total space has been reserved on the heap before more space is needed. For example, `"hello"` would have a length of `5`, but maybe Rust reserved room for `10` bytes total. That would mean,

- Length = `5`,
- Capacity = `10`,

Capacity is the total amount of space allocated on the heap, whereas length is just how much of that memory is actually being used.
