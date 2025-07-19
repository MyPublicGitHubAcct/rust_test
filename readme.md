# Rust Test

## Basics

The following are used below.

- [Rust](https://www.rust-lang.org) language home page.
- [Rust book](https://doc.rust-lang.org/stable/book/) shows the basics of the language, from the developer.
- [Cargo book](https://doc.rust-lang.org/cargo/print.html) is helpful to understand working with Rust.
- [Homebrew](https://brew.sh) is  a package manager for Mac OS.
- [Zed Editor](https://zed.dev) an editor built in Rust which can be used for Rust, C/C++, Javascript, etc.
- [Rustlings](https://rustlings.rust-lang.org) is a library used for learning the Rust language.
- [Learning Rust](https://cglab.ca/~abeinges/blah/too-many-lists/book/README.html) is a blog post describing one way to understand Rust.
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

#### To compile a file or project, the following commands can be used.

```zsh
rustc your_file_name.rs
cargo <PROJECT NAME> build
cargo <PROJECT NAME> b
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

#### Linting

```zsh
cargo clippy
```

## Data Types
