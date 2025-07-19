mod data_types {
    pub mod compound_types;
    pub mod primitive_types;
}

/**
 * main.rs
 * Build and run like "cargo run main.rs"
 * Build like "rustc main.rs"
 * Build like "cargo build main.rs"
 * Run like "./main"
 */
fn main() {
    println!("Hello, world!");
    data_types::compound_types::print_compound_examples();
    data_types::primitive_types::print_primitive_examples();
}
