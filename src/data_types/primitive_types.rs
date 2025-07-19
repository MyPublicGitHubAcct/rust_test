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
