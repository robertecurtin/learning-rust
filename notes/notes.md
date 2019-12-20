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
```
let mut guess = String::new();
... // read input into guess
let guess: u32 = guess.trim().parse().expect("Numerical input expected");
```
`trim` removes trailing newline, `parse` parses string into a number, `: u32` tells us what kind of number to expect, which informs `parse`'s output. Since it could fail to cast, it returns a `Result` type, and must be `expect`ed in order to handle errors
`loop { ... }` creates an infinite loop that can be exited with `break`, `continue` skips to the next iteration of the loop

### Chapter 3
