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

### Chapter 7

"
    Packages: A Cargo feature that lets you build, test, and share crates
    Crates: A tree of modules that produces a library or executable
    Modules and use: Let you control the organization, scope, and privacy of paths
    Paths: A way of naming an item, such as a struct, function, or module
"

Rust uses Packages, Crates, Modules, and Paths

#### Package
Defined in `Cargo.toml`

Allows you to build / test / share a crate or crates that provide a set of functionality.

I imagine there is usually one per git repo.

Can contain zero or one library crates and/or any number of binary crates.

#### Crate
A binary or library.

`src/main.rs` is the crate root of a binary crate.

`src/lib.rs` is the crate root of a library crate.

If you want multiple binary crates, place files in `src/bin`.

You can combine binary and library crates in the same package.

Crates are namespaced, so including `rand` as a dependency means you can access its `Rng` trait using `rand::Rng`

Library crates that expose a function as part of their public API must use the `pub` keyword

#### Module
Define a module with `mod name_of_mod {...}`

Load a module from a file of the same name using `mod name_of_mod;` (note the `;`)

Modules can hold definitions of items like structs / enums / traits / functions, and can contain other modules

Modules form a module tree, with the crate root (`main.rs`/`lib.rs`) as the root

Module tree structure is analogous to a file system

#### Path
Your path is how you navigate the module tree, similar to an os path. The book suggests using absolute paths. "The decision should depend on whether you’re more likely to move item definition code separately from or together with the code that uses the item."

Absolute paths start at crate:

`crate::front_of_house::hosting::add_to_waitlist();`

Relative paths start at the current path:

`(from front_of_house) hosting::add_to_waitlist();`

You can use `super` to go up a level in the path, like `..`:

`(from crate::other_module) super::hosting::add_to_waitlist();

Everything is private by default in Rust :party:, `pub` keyword makes it public. This includes modules within modules.

Structs can be made public with `pub`, but you still need to make their fields public:
```rust
pub struct AStruct {
  pub a_field: String,  // I'm available!
  another_field: String // I'm seeeecret
}
```

Making an enum public, however, makes all of its variants public:
```rust
pub enum AnEnum {
  A, // I'm available!
  B, // I'm available!
}
```

`use` allows you to bring a path into the local scope, and is analogous to a symbolic link.

By convention, this is handled differently for functions and items. There's no strong reason for this.

For functions, it's idiomatic to bring the parent module into scope with `use`:

```rust
use crate::some_module::some_function // ugh

pub fn do_a_thing() {
  some_function(); // this works, but don't do it
}
```
vs
```rust
use crate::some_module // yay

pub fn do_a_thing() {
  some_module::some_function(); // oh, that's where that function came from
}
```

Whereas for structs, enums, and other items, it's idiomatic to bring them into scope by `use`-ing the full path
```rust
use std::collections::HashMap;

pub fn do_a_thing() {
  let mut map = HashMap::new();
}
```

Obviously, if you are using two items with the same name, you need to namespace them or use `as`.

```rust
use std::fmt::Result as FmtResult; // their example had this as "Result" instead of having paired names :|
use std::io::Result as IoResult;
```

You can expose private included items using `pub use`.

Adding a dependency to `Cargo.toml` tells it to go to `crates.io` and get that crate to include in the project

You can use nested paths to bring multiple things into scope:
```rust
use std::io;
use std::io::Read;
use std::io::Write;
```
vs
```rust
use std::io::{self, Read, Write};
```

You can also glob, in case you want to obfuscate your dependencies:
```rust
use std::collections::*;
```


### Chapter 8

Here are all the collections provided by the standard library: https://doc.rust-lang.org/std/collections/index.html

They recommend you pretty much always use `Vec` or `HashMap` unless you _really_ need another data structure

A vector allows you to store a variable number of values of one type (i.e. an expandable array)

A string is a collection of characters

A hash map associates keys with values (i.e. a dictionary)

You can use `&v[i]` or `v.get(i)` to index a vector. `v.get(i)` returns `Some(&element)`, so use it when you want to handle indexing errors instead of crashing.

You can init a vector with `vec![1, 2, 3]`

`for i in &v {...}`

If you want to store multiple types in a vector, abuse the fact that an Enum is a type, and can hold different types

`str` (string slice) is the only string type, although we usually use `String`, which is a growable string (like a vector)

Strings are UTF-8 encoded, so they support different languages

`s.len()` gives byte length, not string length, be careful when working with extended characters

You can't index into strings because UTF-8, instead, you slice them

Hashmaps copy any values placed into them with the Copy, and therefore take ownership

`for` loops over a hash map have no particular order


### Chapter 9
Unrecoverable errors are bugs, like accessing an array out-of-bounds. `panic!` is used for these.

Recoverable errors are expected possible problems, such as file not found errors. The `Result<T, E>` type is used for these.

`panic!` prints a failure message, unwinds and cleans up the stack, then quits. You can switch this behavior to ending the program without cleaning up by adding `panic = 'abort'` to the relevant `[profile]` sections of `Cargo.toml`. This reduces binary size.

You can set `RUST_BACKTRACE` to `1` to get a backtrace (`RUST_BACKTRACE = 1 cargo run`), as long as you have debug symbols enabled (which they are for `cargo build/run` without `--release`)

Result is defined as
```rust

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

You can use `match` to handle `Result`s:
```rust
let f = File::open("foo.txt");
let f = match f {
  Ok(file) => file,
  Err(error) => {
    panic!("Error opening: {:?})", error)
  };
}
```

You can also match on different error types:
```rust
let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("foo.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
```

In practice, you'll probably use closures (which are in chapter 13), which will look like this:
```rust
let f = File::open("foo.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("foo.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
```

`unwrap()` will take a `Result` and either return the value in `Ok` or call `panic!` with the error message

`expect("...")` does the same as `unwrap`, but allows you to provide your own error message

You can use the `?` operator to propagate errors when your function returns a `Result`

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("foo.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
You can even chain `?`s
```rust
    File::open("foo.txt")?.read_to_string(&mut s)?;
```
Of course, when you actually read from files, you'll do this:
```rust
fs::read_to_string("foo.txt")
```

Here's a trick specific to using `?` in `main`:
```rust
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

Returning `Result` should be the default case for a function that might fail

Calls to `unwrap` in examples or in prototype code can indicate where actual error handling would be

In tests, you want to use `unwrap` / `expect` so that it fails loud and clear

If you know that your code will never fail, it makes sense to call `unwrap`

You should `panic` whenever your code ends up in a bad state, meaning when invalid value, contradictory values, or missing values are passed, plus one or more of these:
- The state isn't one that's expected to happen occasionally
- Your code relies on not being in the bad state
- There's not a good way to encode this info in the type

When failure is expected, return a `Result`. When it isn't, then `panic`

You can also use a custom type to ensure that you can't ever recieve a non-valid value, and put validations in the type itself:

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

### Chapter 10
Generics allow you to take in parameters of some generic type

Traits allow you to define behavior in a generic way, which you can then combine with generic types to only allow a class of types that have a particular trait

Lifetimes are generics that give the compiler information about how references relate to each other, and allow us to borrow values and tell the compiler how to check that they are valid

Function with a generic data type:
```rust
fn largest<T>(list: &[T]) -> T { ... }
```
Struct with a generic data type:
```rust
struct Foo<T, U> { a: T, b: U }
```
Enum with a generic data type:
```rust
enum Option<T> { Some(T), None }
```

You can also have `impl` that work on specific types, but not others

Rust's implementation of generics uses "monomorphization" to make sure code doesn't run slow when using generics

It complies separate versions  of the generic code for each type that you actually use, like you'd expect. This will impact code size

Traits are similar to interfaces in other languages, they define shared behavior between types

Each type implementing a trait must provide its own custom behavior for the trait
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```
Then when you implement it:
```rust
impl Summary for SomeStruct {
  fn summarize(&self) -> String { ... }
}
```

You can also add default implementations. Note that default implementations can refer to other methods of the trait that may not have default implementations
```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
...
impl Summary for SomeStruct {}
```

You can't implement external traits on external types, in order to ensure that you can't have conflicting implementations for the same trait and type

You can also request that a trait be available for your function (or state that the value you return implements a trait):
```rust
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
which is syntactic sugar for a longer form called a _trait bound_:
```rust
pub fn notify<T: Summary >(item: T) { ... }
```

If you want to ensure that multiple parameters have the exact same type, you have to use a trait bound. Using `impl Trait` would allow multiple parameters to have different types that all implement that trait

Use `<T: Trait1 + Trait2>` or `impl Trait1 + Trait1` to require multiple traits

You can use a `where` clause when things get crazy:
```rust
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

Note that you can only use `impl Trait` if you're returning a single type across all your condition's branches

_Blanket implementations_ allow you to implement a trait on any type that satisfies its trait bounds, which is how anything that implements the `Display` trait gets access to `to_string` from the `ToString` trait:
```rust
impl<T: Display> ToString for T { ... }
```

Every reference has a _lifetime_, the scope for which it is valid. Like types, they're typically inferred. However, it may not be clear which variable's lifetime the reference should come from.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x // is the output lifetime from here?
    } else {
        y // or from here?
    }
}
```
In the code above, the output reference's lifetime will last as long as the shorter of the two inputs

In some special cases (called _lifetime elision rules_), you don't need to specify lifetime, because the complier can determine it

Here are the three current rules, more may come later:
The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32)`; a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; and so on.

The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.

The third rule is if there are multiple input lifetime parameters, but one of them is `&self` or `&mut` self because this is a method, the lifetime of `self` is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

`'static` is a special lifetime that lives for the duration of the program. All string literals have the `'static` lifetime


### Chapter 17
`pub` keyword allows us to make modules / types / functions / methods public



### Chapter 19
You can create type synonyms if you want them to be interchangable, but this removes type validation:
```rust
type Kilometers = i32;
```

The "never type" `!` can be used for functions that never return, like main.

You can use a trait object to return a closure:
```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

#### Function pointers
Functions coerce to the `fn` type, which is the function pointer type
```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}
```

#### Macros
There are four types of macros, one that is declarative (`macro_rules!`) and three that are procedural

`macro_rules!` is the main form of macros, and allow you to make something similar to a match expression. They can also iterate over provided arguments

Here are some fun examples: https://doc.rust-lang.org/reference/macros.html

Procedural macros act like functions, taking code as an input and producing code as an output

Currently, procedural macro definitions must reside in their own crate with a special crate type

These probably let us do nifty stuff, but I don't need to know it yet

