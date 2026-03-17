# Slices

A **slice** is a borrowed view into contiguous data somewhere in memory. Slices are typically
used behind [references](references.md), such as `&[T]` or `&str`. They also do not have ownership
of the data they point to. This means a slice is never a copy of the data nor an owner of the data.
All slices follow strict rules enforced by the borrow checker:

- A slice depends on the underlying data it refers to remaining valid
- A slice cannot outlive the data it refers to
- A slice is a borrow, meaning it cannot own the data it refers to
- A slice is a range spanning a certain length of the underlying data

### Usage

A slice is declared via wrapping a [range expression](../basics/ranges.md) within `[]` square
brackets. Prefixing a collection with the `&` operator creates a borrow of that slice which points
to a collection. For example:

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[..];
}
```

Here `a` is an array bound to 5 integer elements. Next, `slice` is declared with the type `&[i32]
`, indicating a borrow of a range of integers. `slice` is bound to a borrow of `a`, followed by
a slice `[..]`, indicating a range expression within a collection.

- `a[..]` is a slice, with the `..` [full-range](../basics/ranges.md#full-range) expression,
  meaning a range that spans the entire length of the array (all elements).
- `&` borrows that range as a slice.

## Slice Types
