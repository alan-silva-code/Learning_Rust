// Primitive data types
// Reference: 
// https://doc.rust-lang.org/book/ch03-02-data-types.html

fn main(){
    let x: i32 = -34;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // Float
    let pi: f64 = 3.14;
    let euler: f32 = 2.718;

    println!("Value of pi {}", pi);
    println!("Value of Euler's number {}", euler);

    // Boolean Values: true, false
    let is_raining = true;
    let mut is_sunny = true;

    if is_raining {
        is_sunny = false;
    }

    println!("Is raining? {}", is_raining);
    println!("Is sunny? {}", is_sunny);

    // Charachter Type - char
    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);
}
