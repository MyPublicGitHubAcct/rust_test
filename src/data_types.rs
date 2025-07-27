use std::collections::HashMap;
use std::fmt;

/* ================ Primitive data types ================
 * Integer signed types = i8, il6, i32, etc.
 * Integer unsigned types = u8, u16, u32, etc.
 * Float types = f32, f64.
 * Boolean = bool
 * Character = char
 */
pub fn print_primitive_examples() {
    let x: i32 = 42;
    let y: u64 = 100;
    println!("Signed Integer: {x}");
    println!("Unsigned Integer: {y}");

    let some_number: f64 = 13.14567;
    println!("Value of some_number: {some_number}");

    let is_snowing: bool = true;
    println!("Is it snowing? {is_snowing}");

    let letter: char = 'a';
    println!("First letter of the alphabet is {letter}.");
}

/* ================ Compound data types ================
 * Array, Tuple, Slice, or String
 * Strings are mutable, String Slices (&str) are not.
 */
pub fn print_compound_examples() {
    let numbers_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers array: {numbers_array:?}");

    let fruit_array: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits array: {fruit_array:?}");
    println!("Fruits array 2nd element: {}", fruit_array[1]);

    let human_tuple: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human tuple: {human_tuple:?}");

    let mixed_tuple = ("Text", 23, true, [1, 2, 3, 4, 5]);
    println!("Mixed tuple: {mixed_tuple:?}");

    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Numbers slice: {number_slices:?}");

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal slice: {animal_slices:?}");

    let book_slices: &[&String] = &[
        &"Thinking Mathematically".to_string(),
        &"MaxMSP Book".to_string(),
        &"Gen Book".to_string(),
    ];
    println!("Book slice: {book_slices:?}");

    let mut saying: String = String::from("Hello, ");
    saying.push_str("my friend.");
    println!("Someone says: {saying}");

    let my_string: String = String::from("Hello World.!");
    let my_slice: &str = &my_string[1..5];
    println!("Slice value: {my_slice}");
}

/* ================ Struct ================
 * ========================================
 */
struct User {
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}

pub fn print_email_from_struct() {
    let mut user_one = User {
        active: true,
        user_name: String::from("Steve"),
        email: String::from("user@somedomain.com"),
        sign_in_count: 1,
    };

    println!("--- user_one ---");
    println!("User's name? {}", user_one.user_name);
    println!("User is active? {}", user_one.active);
    println!("User has signed in  {} times", user_one.sign_in_count);
    println!("User email is  {}", user_one.email);

    user_one.email = String::from("differentemail@domain.com");
    println!("User email is now {}", user_one.email);
}

// function that returns a user
fn build_user(email: String, user_name: String) -> User {
    User {
        active: true,
        email,
        user_name,
        sign_in_count: 0,
    }
}

pub fn print_make_user() {
    let mut user_two = build_user("user2@domain.com".to_string(), "Ulysses Hubert".to_string());

    println!("--- user_two ---");
    println!("User's name? {}", user_two.user_name);
    println!("User is active? {}", user_two.active);
    println!("User has signed in  {} times", user_two.sign_in_count);
    println!("User email is  {}", user_two.email);

    user_two.email = String::from("differentuser2@domain.com");
    println!("User email is now {}", user_two.email);
}

/* ================ Enum ================
 * ======================================
 */
enum IpAddrKind {
    V4,
    V6,
}

fn check_enum_type(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("IpAddrKind::V4"),
        IpAddrKind::V6 => println!("IpAddrKind::V6"),
    }
}

pub fn print_enum_type() {
    let four: IpAddrKind = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;

    println!("--- enum checker --");
    check_enum_type(four);
    check_enum_type(six);
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl fmt::Display for IpAddrKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IpAddrKind::V4 => write!(f, "version 4"),
            IpAddrKind::V6 => write!(f, "version 6"),
        }
    }
}

pub fn print_ip_addresses() {
    let home: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback: IpAddr = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("--- ip addresses from a struct ---");
    println!(
        "home's IP address is, {}, and its version is {}",
        home.kind, home.address
    );
    println!(
        "loopback's IP address is, {}, and its version is {}",
        loopback.kind, loopback.address
    );
}

// Assigning types to an enum
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

pub fn print_ip_addresses_from_enum() {
    let my_enum4 = IpAddrEnum::V4(String::from("127.0.0.2"));
    let my_enum6 = IpAddrEnum::V6(String::from("::2"));

    println!("--- ip addresses from an enum ---");
    println!("{}", my_enum4);
    println!("{}", my_enum6);
}

/* ============== Collection: Vectors ===============
 * ==================================================
 */
pub fn print_vector_example() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    println!("Vector v contains: {:?}", v);

    let third_item: &i32 = &v[2];
    println!("The third element is {third_item}");

    let fifth_item: Option<&i32> = v.get(4);
    match fifth_item {
        Some(fifth_item) => println!("The fifth item is {fifth_item}"),
        None => println!("There is no fifth element."),
    }
}

/* ============== Collection: Strings ===============
 * ==================================================
 */
pub fn print_string_example() {
    let s: String = "something".to_string();
    println!("The value of s = {s}");

    let s: String = String::from("something else");
    println!("The value of s = {s}");

    let mut s: String = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("The value of s = {s}");

    let new_string: String = s + "\nand this too.";
    println!("The value of new_string = {new_string}");
}

/* ============== Collection: Hash Maps==============
 * ==================================================
 */
pub fn print_hash_map_example() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score for team {team_name} from the hash map = {score}.");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
