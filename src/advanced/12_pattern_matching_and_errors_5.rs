fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by 0".to_string())
    } else {
        Ok(a/b)
    }
}
 
fn main() {
    match divide(10, 2) {
        Ok(result) => println!("Result of division is {}", result),
        Err(error) => println!("Error: {}", error),
    }
    match divide(10, 0) {
        Ok(result) => println!("Result of division is {}", result),
        Err(error) => println!("Error: {}", error),
    }
}