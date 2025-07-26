/*
 * Error Handling
 */
fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

pub fn print_error_handling_example_one(numerator: f64, denominator: f64) {
    println!("+++++++one+++++++");
    println!("numerator = {}", numerator.to_string());
    println!("denominator = {}", denominator.to_string());
    let res = divide_option(numerator, denominator);
    match res {
        Some(x) => println!("Result: {x}"),
        None => println!("Error!"),
    }
}

fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("You cannot divide by 0.".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

pub fn print_error_handling_example_two(numerator: f64, denominator: f64) {
    println!("+++++++two+++++++");
    println!("numerator = {}", numerator.to_string());
    println!("denominator = {}", denominator.to_string());
    let res = divide_result(numerator, denominator);
    match res {
        Ok(result) => println!("Result: {}", result.to_string()),
        Err(err) => println!("Error: {}", err),
    }
}
