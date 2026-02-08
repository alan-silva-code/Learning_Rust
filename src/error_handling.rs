// Error Handling techniques

// Approach 1
/*  
enum Option<T>{ // Define the generic option type
    Some(T), // Represents a value
    None, // Represents no value
}
*/

// Approach 2
/*
enum Result<T, E>{ // Define the generic Result type
    Ok(T), // Represents a value
    Err(E), // Represents an error
}  // Prevent a compile time
*/
 
fn divideOption(numerator: f64, denominator: f64) ->Option<f64>{
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}


fn divideResult(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by 0".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn main(){
    /* 
    let result = divideOption(91.0, 0.0);
    match result{
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero!")
    }
    */

    match divideResult(100.56, 67.98){
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

}