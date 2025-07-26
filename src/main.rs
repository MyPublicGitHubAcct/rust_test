mod data_types;
use data_types::print_primitive_examples;

mod function;
use function::functions;

mod libs;
use libs::ownership::ownerships;
use libs::reference::references;

/**
 * main.rs
 * Build and run like "cargo run main.rs"
 * Build like "rustc main.rs"
 * Build like "cargo build main.rs"
 * Run like "./main"
 */
fn main() {
    println!("Hello, world!");
    data_types::print_compound_examples();
    print_primitive_examples();
    data_types::print_email_from_struct();
    data_types::print_make_user();
    functions::print_function_examples();
    ownerships::print_ownership_examples();
    references::print_reference_examples();
}
