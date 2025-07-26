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
 * Example struct & use
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
