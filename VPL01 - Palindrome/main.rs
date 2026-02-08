use std::io;

fn string_manipulation() -> String {
    let mut word = String::new();
    io::stdin()
    .read_line(&mut word)
    .expect("Error! Please try again.");

    return word.trim().split_whitespace().next().unwrap_or("").to_string().to_uppercase();
}

fn palindrome(){
    let result = string_manipulation();
    let mut length: usize = result.len();
    let mut i: usize = 0;
    while i == length {
    let u: char = result.chars().nth(i).unwrap();        
    if u == result.chars().nth(length).unwrap() {
            length = length - 1;
            i = i + 1;
        } else {
            println!("It is not a palindrome!");
            break;
        }
    }
    println!("It's a palindrome.");
}

fn main(){
    println!("Type something: ");

    palindrome();
}