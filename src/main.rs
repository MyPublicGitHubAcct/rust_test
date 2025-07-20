mod data_types {
    pub mod compound_types;
    pub mod primitive_types;
}

mod function {
    pub mod functions;
}

mod ownership {
    pub mod ownerships;
}

mod reference {
    pub mod references;
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
    function::functions::print_function_examples();
    ownership::ownerships::print_ownership_examples();
    reference::references::print_reference_examples();
}
