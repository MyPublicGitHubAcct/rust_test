fn person_information(name: &str, age: u32, height: f32) {
    println!(
        "Person's name is {}. They are {} years old and {} cm tall.",
        name, age, height
    );
}

pub fn print_function_examples() {
    person_information("Robert Fripp", 79, 170.0);
}
