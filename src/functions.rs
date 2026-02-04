// Functions
// an function / variables should be written in snake case
fn main(){
    hello_world();
    tell_height(187);
    human_id("Maria", 23, 178.87);
    let _x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };

    println!("Result is: {}", _x);
    
    let y: i32 = add(6, 4);
    println!("Value of y is: {}", y);

    // Calling the function
    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi: f64 = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);

}

// functions returning values
fn add(a: i32, b:i32) -> i32{
    a + b
}

fn hello_world(){
    println!("Hello, world!")
}

fn tell_height(height: u32){
    println!("My height is {}", height);
}

fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, and my height is {} cm.", 
    name, age, height);
}

// Final example
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64{
    weight_kg / (height_m * height_m)
}