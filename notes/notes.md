Rust uses .rs extension, with lower_snake_case_naming.rs

`rustc` compiles when not using cargo

`cargo build` builds a cargo project into `target/debug/project_name`

`cargo run` builds and runs

`cargo check` checks for compilation errors, but doesn't compile

`cargo build --release` compiles with optimizations, which increases build time

Libraries should git ignore `Cargo.lock`, binaries should check it in

`Cargo.lock` describes the state at a particular time, and allows for deterministic builds

the _prelude_ is the list of things that Rust automatically imports, notably traits

other libraries may define their own preludes, which are not automatically `use`'d, but can be convenient for importing a bunch o' stuff

`macro!` invokes `macro`, because macros are exciting!

`let` = immutable var, `let mut` = mutable var, `&var` = immutable reference, `&mut var` = mutable reference

`// comment`

`String` is a standard library string that goes on the heap

`::` indicates associated function implemented on a type (static method)

`use std::io` means you can write `io::stdin().readline(...)` instead of`std::io::stdin().readline(...)`

generic `Result` is enum of `Ok` / `Err`, `Err` contains info on why it failed, there are lots of variants by different libraries for some reason

`println!("a = {}, b = {}", a, b)`

Rust uses [Semantic Versioning](https://semver.org/) (`major.minor.patch`)

`Cargo.toml` manages dependencies, `0.5.5` == `^0.5.5`, meaning "any version with a public API compatible with 0.5.5"

`cargo update` will update to latest patch with same major and minor version, so `0.5.5` -> `0.5.6`, but not -> `0.6.0`

How do you find and create crates? https://doc.rust-lang.org/cargo/ and http://doc.crates.io/crates-io.html

`cargo doc --open` will build and open documentation for all your dependencies :O

`Ordering` is used for comparisons

`match` is used as a switch for enumerations, and is made up of _arms_

An _arm_ consists of a pattern and code to run

Rust has a strong, static type system and also type inference

When converting between types, you can shadow previous values:

```rust
let mut guess = String::new();
... // read input into guess
let guess: u32 = guess.trim().parse().expect("Numerical input expected");
```

`trim` removes trailing newline, `parse` parses string into a number, `: u32` tells us what kind of number to expect, which informs `parse`'s output. Since it could fail to cast, it returns a `Result` type, and must be `expect`ed in order to handle errors

`loop { ... }` creates an infinite loop that can be exited with `break`, `continue` skips to the next iteration of the loop

### Chapter 3

Using `const` instead of `let` declares a constant, and requires that you define the type:

```rust
const SOME_VALUE: u32 = 100;
```

Constants can be declared in the global scope

You only need to declare a type when it can't be inferred. Rust's nifty compiler errors will help you here.

#### Types
Integer: `i/u` `8/16/32/64/128/size`

Float: `f32` / `f64`

Character: `char`, written as `'z'`

Tuple: `tup`, written as `(i8, char, f32)`
```rust
let tup = (1, 1.8, 'z')
let (a, b, c) = tup
or
let tup: (i32, f32, char) = (1, 1.8, 'z')
let a = tup.0
let b = tup.1
```

Array: `[type; length]` `[i8; 3] = [1, 2, 3]`

Can use `_` as a visual separator: `12_345` == `12345`

Name|Example
---|---
Hex | 0xFF
Octal | 0o77
Binary | 0b0101_0110
Byte | b'A'

Rust will throw runtime errors if you index an array out of bounds

Rust uses snake_case for function and variable names

Rust requires type declarations for function definitions, which means they can be inferred elsewhere

Functions contain statements and expressions. Statements do not return a value, expressions evaluate to a value

```rust
// This is valid:
let x = 5;
let y = {
  let x = 3;
  x + 1
}; // y == 4
```

Expressions terminated with a semicolon become a statement

### Chapter 4

`let s == "hello";` `s` is a string literal, has a known length, and is allocated on the stack

`let s = String::from("ss");` `s` is of type `String`, is allocated on the heap, and is thus dynamic

Memory is returned when the variable *owning* it goes out of scope

Rust calls `drop` automatically, which returns memory

Simple data types with fixed size can be copied:
```rust
let x = 5;
let y = x;
let z = x; // this is fine
```

References are moved rather than copied, and can't be referenced after that point:
```rust
  let x = String::from("hi");
  let y = x;
  let z = x; // compilation error!
```

Objects consist of a pointer, data length (current length), and capacity (total memory allocated)

You can deep copy with `clone`:
```rust
  let x = String::from("hi");
  let y = x.clone();
  let z = x; // this is fine
```

Types with the `Copy` trait don't need to be cloned. These include:
- Integer types
- bool
- Floating point types
- char
- Tuples and arrays that contain types with the `Copy` trait

Passing a value to a function also passes ownership of the variable, and will move / copy accordingly
```rust
  let x = String::from("hi");
  some_function(x);
  let y = x; // compilation error!
```

Returning values also transfers ownership
```rust
fn return_something() -> String {
  let x = String::from("hi");
  x // returned, but memory not freed, how tidy
}
```

You can also use references when calling functions, dereference is & and reference is *, like in c

Rust calls using references "borrowing"

You can't modify something you've borrowed:

```rust
  fn length(s: &String) -> usize {
    s.push_str("oh no"); // compilation error!
  }
  let x = String::from("foo");
  println!("{}", length(&x));
```

You can fix the above code using `mut`, but you can only have one mutable reference in a given scope (only one person can borrow it at once). This prevents data races.

```rust
  let x = String::from("foo");
  let r1 = &mut x;
  let r2 = &mut x; // compilation error!
```

This is fine, though, because the brackets ensure we won't have _simultaneous_ references

```rust
  let x = String::from("foo");
  {
    let r1 = &mut x;
  }
  let r2 = &mut x; // this is fine
```

Likewise:
```rust
  let x = String::from("foo");
  let r1 = &x; // this is fine
  let r2 = &x; // everything is fine
  let r3 = &mut x; // compilation error!
```

Rust prevents you from having a _dangling pointer_ (pointer referencing memory that has been freed) because of these rules somehow, I haven't bothered to think through how yet

_Slices_ allow you to reference a contiguous sequence of elements within a collection. String slices are `&str`, array slices are &[i32]

Slices tie indices to the collection they are connected to

String literals *are* string slices `&str`

### Chapter 5

Structs are structs

Tuple structs are basically tuples that are defined as types so they can be considered when the compiler validates types

You can define structs without fields, called unit-like structs, can be useful when you have types with traits but no data stored

Rust doesn't have the `->` operator, it automatically references / dereferences because it knows the expected type


### Chapter 6

You can store data with different enumerated values

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

`Option` is used when a value could be something or nothing. Rust doesn't have `null`. You have to convert an `Option<T>` to `T` before you can perform `T` operations. This protects you from unsafe assumptions.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`match` lets you compare a value against patterns, checking the top pattern first and then each subsequent one. You will frequently omit {} on the right hand side, since a value is an expression

`_` is the default pattern

```rust
match value {
  Value::A => {
    1
  },
  Value::B => 2,
  _ => 0 // default
}
```

`if let` is syntatic sugar for a match that takes one case and ignores defaults

```rust
if let Some(3) = some_value {
  do_a_thing();
}
```

