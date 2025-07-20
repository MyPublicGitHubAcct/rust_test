# Rust Test

## Basics

The following are used below.

- [Rust](https://www.rust-lang.org) language home page.
- [Rust book](https://doc.rust-lang.org/stable/book/) shows the basics of the __Rust__ language, from the developer.
- [Cargo book](https://doc.rust-lang.org/cargo/print.html) is helpful to understand working with Rust.
- [Homebrew](https://brew.sh) is  a package manager for Mac OS.
- [Zed Editor](https://zed.dev) an editor built in __Rust__ which can be used for __Rust__, __C/C++__, __Javascript__, etc.
- [Rustlings](https://rustlings.rust-lang.org) is a library used for learning the __Rust__ language.
- [Learning Rust](https://cglab.ca/~abeinges/blah/too-many-lists/book/README.html) is a blog post describing one way to understand __Rust__.
- [Introductory video](https://youtu.be/rQ_J9WH6CGk?si=fIP3MCsNvukSm6bH) by BekBrace.

### Install Rust

```zsh
brew install rust
brew install rustup
brew install rustc-completion
```

### Create a new project & open with Zed

```zsh
cargo new <PROJECT NAME>
zed <PROJECT NAME>
```

### Compile & Run

#### Before building, you should check the code as this is faster.

```zsh
cargo check
```

#### To compile a file or project, the following commands can be used.

```zsh
rustc your_file_name.rs
cargo <PROJECT NAME> build
cargo <PROJECT NAME> b
```

#### To clean a project

This will remove the compiled file and nay others used to build the project.

```zsh
cargo clean
```

#### To execute a file or project, the following commands can be used.

```zsh
./your_file_name.rs
cargo <PROJECT NAME> run
cargo <PROJECT NAME> r
```

#### To test a project, the following commands can be used.

```zsh
cargo <PROJECT NAME> test
cargo <PROJECT NAME> t
```

## Rust conventions

The [Rust style guide](https://doc.rust-lang.org/nightly/style-guide/) describes the current conventions to use, and what the linter, _clippy_ will enforce.

All __functions__ and __variables__ should be written in __snake case__ (lower case words separated by an underscore) or __kebab case__ (lower case words separated by a hyphen).

__snake case__: ```_hello_world_```.

__kebab case__: ```hello-world```.

## Linting

```zsh
cargo clippy
```

## Data Types

Likc __C/C++__, __Rust__ is a strongly typed language. Also like __C/C++__, __Rust__ makes use of _type interference_.  In __C/C++__ the _auto_ keyword was was introduced for backward compatibility and often relies on the initialization expression.

__Rust__ has no such need, so initializing a variable like ```let x = 5;``` will cause the compiler to use this statement and use of ```x``` to infer the data type.

However, it is a common practice to define the data type manually like ```let x: i32 = 5;```.

## Ownership

Memory allocated needs to be returned when it is no longer needed for it's purpose. Some languages include "garbage collection" which can dramatically slow done processsing and is not adequate for real-time applications e.g., DAWs (signal processing), modular synthesis (embedded).

This can also be a security issue, as expressed in [this US government paper](https://bidenwhitehouse.archives.gov/wp-content/uploads/2024/02/Final-ONCD-Technical-Report.pdf).

### The three rules of Ownership

1. Each value in __Rust__ has an owner.
2. There can be only one owner at a time.
3. When an owner goes out of scope, the value will be dropped.

The example in ```print_ownership_examples()``` (shown below) shows that refrences can be used to read and use the value of s1 for a calculation.

```rust
fn main() {
    let s1 = String::from("RUST");
    let len  = calculate_length(&s1);
    println!("Length of {s1} is {len}")
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

To _transfer the ownersip of the variable_, one would need to transfer the value to another variable, like below. Attempting to access the value of ```s1``` in this case would result in an error.

```rust
let s1 = String::from("RUST");
let s2 = s1;
```

## Borrowing and References

_Safety_ refers to the prevention of things like null pointer deferencing, dangling pointers, buffer overflows, and data races. _References_ are used to avoid many of these types of challenges.

One can make a _Reference_ by _Borrowing_ a value from the owner. _References_ in __Rust__ can be mutable or immutable.

You can not _borrow_ as both mutable and immutable at the same time. Further, you can have _only one mutable refrence_, or _many immutable references_. This is often handled by ensuring that each use of a variable is in its own scope e.g., function.

### An Immutable Reference

```rust
let _x: i32 = 5;
let _r = &i32 = &_x;
*_r += 1;  // This will result in an error as _r is an immutable reference
```

### A Mutable Reference

```rust
let mut _x: i32 = 5;
let _r = &mut = _x;
*_r += 1;  // This will not result in an error as _r is a mutable reference
```

## Variables and Mutability

## Constants

## Shadowing

## Control Flow

## Looping Mechanisms

## Structs

## Enums

## Error Handling

## Collection Types

### Vectors

### UTF-8 Strings

### Hash Maps
