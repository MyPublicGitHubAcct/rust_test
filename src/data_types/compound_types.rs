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
