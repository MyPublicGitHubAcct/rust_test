fn person_information(name: &str, age: u32, height: f32) {
    println!("Person's name is {name}. They are {age} years old and {height} cm tall.",);
}

pub fn print_function_examples() {
    person_information("Robert Fripp", 79, 170.0);
}
