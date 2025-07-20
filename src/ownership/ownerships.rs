fn calculate_length(s: &str) -> usize {
    s.len()
}

pub fn print_ownership_examples() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of {s1} is {len}")
}
