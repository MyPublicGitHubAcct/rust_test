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
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) is set of examples.
- [Rust Projects - Redis](https://leanpub.com/rustprojects-redis) is an example recreation of Redis.
- [Introductory video](https://youtu.be/rQ_J9WH6CGk?si=fIP3MCsNvukSm6bH) by BekBrace (Amir Bekhit).
- [Rust Audio](https://rust.audio/) is a list of crates useful for audio processing.
- [Crates.io](https://crates.io/) is a directory of crates.
- [Xilem](https://github.com/linebender/xilem) is  GUI framework for Rust.
- [Leptos](https://leptos.dev/) is a web framework for Rust.
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/intro.html) is a list of design patterns intended to provide standard ways of accomplishing common tasks.
- [Embassy framework](https://github.com/embassy-rs/embassy) for embedded.

### Install & Update Rust

Download & run the script that installs the rustup tool. This will install cargo, clippy, rust-docs, rust-std, rustc, and rustfmt.

```zsh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Check that the installation worked.

```zsh
rustc --version
```

Update Rust

```zsh
rustup update
```

Uninstall Rust

```zsh
rustup self uninstall
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

#### To format Rust files

This will remove the compiled file and nay others used to build the project.

```zsh
cargo fmt
```

#### To execute a file or project, the following commands can be used.

The cargo commands will also build the project if it has changed, whereas the __./__ command will not.

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

All _functions_ and _variables_ should be written in __snake case__ (lower case words separated by an underscore) or __kebab case__ (lower case words separated by a hyphen).

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

When a _variable_ is immutable, once a value has been bound to a name, it cannot be changed. A _variable_ can be made to be mutable with the ```mut``` keyword. For example:

```rust
let mut _a: i32 = 5;
```

By convention, _variables_ that are unused should start with an underscore. The compiler will give a wanrning but will still compile the code if the underscore is missing.

If there are no situations where a _variable_ that is declared using the ```mut``` keyword is changed, the compiler will complain. So, remove this keyword in that situation. Better, don't use it until you have a situation requiring its use.

## Constants

_Constants_ are similar to immutable variables in that the values they own are not changeable. Unlike _variables_, they cannot be changed to be mutable by using the ```mut``` keyword. Additionally, one must declare the type of a _constant_. By convtention, _constants_ are capitalized. An example:

```rust
const Y: i32 = 10;
```

Unlike _variables_, _constants_ can be declared in the global namespace.

## Shadowing

In __Rust__, one can reuse a _variable_ name and bind it to a different value, effectively changing its value or type. In that case, the first _variable_ is _shadowed_ by the second _variable_. The compiler will see the second _variable_ only. _This does allocate memory on the stack_.

For example, changing value:

```rust
let x = 5;
let x = x + 1;
```

Another example, changing type:

```rust
let mut age = String::new();
println!("Type your age: ");
io::stdin().read_line(&mut age).expect("Error");
let age:u8 = age.trim().parse().expect("Error");
```

Another...

```rust
let spaces = "   ";
let spaces = spaces.len();
```

These are commonly used when the _variable_ will be used as a parameter in a function call or a type change is needed. This is also useful when it is not desirable to make the _variable_ mutable.

## Comments

__Rust__ allows for __C/C++__-style comments.

```rust
// single line comment

/* block comment
 * Multiple line comments
 * are fun.
 */
```

## Control Flow

### If Else expressions

```rust
// if, else
let age: u16 = 18;
if age >= 18 {
    println!("You can drive a car.");
} else {
    println!("You cannot drive a car.");
}

// if, else if, else
let number: u16 = 6;
if number % 4 == 0 {
    println!("number is divisible by 4.");
} else if number % 3 == 0 {
    println!("number is divisible by 3.");
} else if number % 2 == 0 {
    println!("number is divisible by 2.");
} else {
    println!("number is divisible by 2, 3, or 4.");
}
```

### Use If in a Let statement

```rust
let condition = true;
let number = if condition {
    5
} else {
    6
};
println!("Number: {number}");
```

## Looping Mechanisms

The _loop_ keyword used without parameters will run forever. The _break_ expression is used to stop the loop.

### Loop expressions

When ```counter``` is equal to 10, this will return 20.

```rust
let mut counter = 0;
let result = loop {
  counter += 1;
  if counter == 10 {
      break counter * 2;
  }
};
println!("The result is {result}")
```

### Loop Labels for nested loops

#### Loop with break and continue

The _break_ and _continue_ statements apply to the inner-most loop by default. So, _Loop Labels_ are used to make clear what is intended & control the flow of processing. _Loop Labels_ are denoted by a single quote at the start of the name.

In the example below, the first use of _break_ stops the inner loop, and the second stops the outer loop.

```rust
let mut count = 0;
'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;
    loop {
        println!("remaining = {remaining}");
        if remaining == 9 {
            break;
        }
        if count == 2 {
            break 'counting_up;
        }
        remaining -= 1;
    }
};
```

#### While loops

A _while_ loop runs as long as a condition is true.

```rust
let mut number = 3;
while number != 0 {
    println!("{number}");
    number -= 1;
}
```

#### Looping through a collection

Similar to a _for_ loop.

```rust
let a = [1,2,3,4,5,6];
for element in a {
    println!("Number is {element}");
}
```

## Structs

Similar to a _tuple_, a __struct__ is used to hold multiple pieces of data, and those data can be of different types. However, with a __struct__, the data are named values.

```rust
// tuple
let rect: (i32, i32) = (300, 500);

// struct
struct User {
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}

let mut user_one = User {
    active: true,
    user_name: String::from("Steve"),
    email: String::from("user@somedomain.com"),
    sign_in_count: 1,
}
```

### Common uses of structs

```rust
// tuple structs
struct Color(i32, i32, i32);
let black: Color(0,0,0);
let white: Color(255,255,255);

// unit-like struct
struct AlwaysEqual;
let subject: AlwaysEqual = AlwaysEqual;
```

## Enums

An __Enum__ represents a data type similar to a _struct_, but allows each variant to hold different types, and amounts, of data. It can be simple, like

```rust
enum IpAddrKind {
    V4,
    V6
}
```

An __Enum__ can also include types for each. However, doing so will cause a need to define _fmt::Display_ for the enum.

```rust
enum IpAddrEnum {
    V4(String),
    V6(String),
}

impl fmt::Display for IpAddrEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IpAddrEnum::V4(s) => write!(f, "IP4 Address: {}", s),
            IpAddrEnum::V6(s) => write!(f, "IP6 Address: {}", s),
        }
    }
}

let my_enum4 = IpAddrEnum::V4(String::from("127.0.0.2"));
let my_enum6 = IpAddrEnum::V6(String::from("::2"));
```

If printed, this would result in something like:

```shell
IP4 Address: 127.0.0.2
IP6 Address: ::2
```

## Error Handling

The __Rust__ compiler will force you to handle errors, where it perceives the possibility for them. Two _built-in_ error handling types are:

```rust
// Approach 1, the generic Option<T> enum.
enum Option<T>{
    Some(T),  // Represents a value
    None,     // Represents no value
}

// Approach 2, the generic Result<T, E> enum
enum Result<T, E>{
    Ok(T),  // Represents a value
    Err(E), // Represents an error
}
```

To use them, first, make one of them the return type of a function like this.

```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
```

Then, use the return values like this.

```rust
pub fn print_error_handling_result(numerator: f64, denominator: f64) {
    let res = divide(numerator, denominator);
    match res {
        Some(x) => println!("Result: {x}"),
        None => println!("Error!"),
    }
}
```


## Collection Types

### Vectors

### UTF-8 Strings

### Hash Maps
