mod data_types;
use data_types::print_primitive_examples;

mod error_handling;

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
    println!("-------------------------");
    data_types::print_compound_examples();
    println!("-------------------------");
    print_primitive_examples();
    println!("-------------------------");
    data_types::print_email_from_struct();
    println!("-------------------------");
    data_types::print_make_user();
    println!("-------------------------");
    data_types::print_enum_type();
    println!("-------------------------");
    data_types::print_ip_addresses();
    println!("-------------------------");
    data_types::print_ip_addresses_from_enum();
    println!("-------------------------");
    functions::print_function_examples();
    println!("-------------------------");
    ownerships::print_ownership_examples();
    println!("-------------------------");
    references::print_reference_examples();
    println!("-------------------------");
    error_handling::print_error_handling_example_one(1.0, 0.0);
    error_handling::print_error_handling_example_one(1.0, 3.0);
    println!("-------------------------");
    error_handling::print_error_handling_example_two(1.0, 0.0);
    error_handling::print_error_handling_example_two(1.0, 3.0);
    println!("-------------------------");
}
