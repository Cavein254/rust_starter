# Introduction to rust

The language advocates for the use of `snake` casing as
`let my_age:u8`

By default rust uses `i32` to represent integers unless specified
Rust is an immutable language. For example the following code
will generate an error

```rust
let num:i32 = 10;
num = 100;
println!("The new number is {}", num);
```

Instead to achieve this functionality we use `mut` that allows the
values to be mutated.

```rust
let mut num:i32 = 10;
num = 100;
println!("The new number is {}", num);
```

## Calculations

Rust uses similar methods to perform calculation as other programming
languages.
