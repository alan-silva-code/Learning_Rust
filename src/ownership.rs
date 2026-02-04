/* fn main(){
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}.", s1, len);
}
*/

/* 
fn main(){
    let s1 = String::from("RUST");
    let s2 = s1;
    println!("{}", s2);
}
*/

fn calculate_length(s: &String) -> usize{
    s.len()
}

