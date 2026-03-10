# Range Expressions

A **range** expression is a span of values between two bounds. Each variation of a range will produce one of the standard range types in [std::ops](https://doc.rust-lang.org/std/ops/index.html) or [core::ops](https://doc.rust-lang.org/core/ops/).

## Bounds

Each bound is an endpoint of a range expression. The left endpoint is the **lower bound**, and the right endpoint is the **lower bound**. 

```rust
let rng = 1..5;
```

In the range expression, `1` is lower bound, `5` is the upper bound.

A range may have a lower bound, an upper bound, both, or neither. When a bound is present, it may be *inclusive* or *exclusive* depending on the syntax.

### Inclusive Bound

If a bound is **inclusive**, that endpoint is included it the range. For example,

```rust
1..=5
```

This is an inclusive range. It contains all values where:

```rust
value >= start && value <= end
```

This means each value must be greater than or equal to the lower bound, and less than or equal to the end bound. So `1..=5` includes:

```rust
1, 2, 3, 4, 5
```

### Exclusive Bound

If a bound is **exclusive**, that endpoint is not included in the range. For example,

```rust
1..5
```

This range contains all values where:

```rust
value >= start && value <= end
```

In english, each value in the range must be greater than or equal to the lower bound, AND less than the upper bound. So `1..5` includes:

```rust
1, 2, 3, 4
```

This does not include `5` because the upper bound is exclusive (not included in the range).

## Range Expression Variations

There are six different variations of range expressions, each including and excluding differently.

| Syntax        | Name               | Type                         | Range             |
|---------------|--------------------|------------------------------|-------------------|
| start`..`end  | Half-Open Range    | `std::ops::Range`            | start <= x < end  |
| start`..=`end | Inclusive Range    | `std::ops::RangeInclusive`   | start <= x <= end |
| start`..`     | Range From         | `std::ops::RangeFrom`        | start <= x        |
| `..`end       | Range To           | `std::ops::RangeTo`          | x < end           |
| `..=`end      | Range To Inclusive | `std::ops::RangeToInclusive` | x <= end          |
| `..`          | Full Range         | `std::ops::RangeFull`        | unbounded         |

### Half Open Range

It's called **half-open** because only the lower bound is included, while the upper bound is excluded. It uses the `start..end` syntax. So, `0..3` means `0, 1, 2`, but not `3`.

```rust
let range = 0..3;

// same as
let range = std::ops::Range { start: 0, end: 3 };
```

### Inclusive Range

It's called **inclusive range** because both bounds are included. It uses the `start..=end` syntax. So, `5..=10` includes `5`, includes every value before `10`, and includes `10` as well.

```rust
let rng = 5..=10;

// same as
let rng = std::ops::RangeInclusive::new(5, 10);
```

### Range From

It's called **range from** because it literally starts *from* some lower bound, then keeps going upward with no specified upper bound. It uses the `start..` syntax. So, `3..` includes `3`, every value after `3`, and there is no ending bound.

```rust
let rng = 3..;

// same as
let rng = std::ops::RangeFrom { start: 3 };
```

### Range To

It's called **range to** because it goes *to* some upper bound, but does not not include that bound. It uses the `..end` syntax. So, `..5` includes everything less than `5`, but not `5` itself.

```rust
let rng = ..5;

// same as
let rng = std::ops::RangeTo { end: 5 };
```

### Range To Inclusive

It's called **range to inclusive** because it goes *to* an upper bound and *includes* that upper bound. It uses `..=end` syntax. So, `..=5` includes everything less than `5` and `5` itself.

```rust
let rng = ..=5;

// same as
let rng = std::ops::RangeToInclusive { end: 5 };
```

### Full Range

It's called **full range** because it has no lower or upper bound (unbounded range). It uses the `..` syntax. A full range is the entire span, with nothing excluded from either side.

```rust
let rng = ..;

// same as
let rng = std::ops::RangeFull;
```
