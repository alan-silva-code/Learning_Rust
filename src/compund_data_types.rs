// Compound Data Types

fn main(){
    let numbers: [i32; 5] = [0, 1, 2, 3, 4];
    println!("Number Array: {:?}", numbers);
    println!("The first number on Array: {}", numbers[0]);

    let fruits: [&str; 5] = ["strawberry", "pineapple", "orange", "melon", "tangerine"];
    println!("The favorite fruits are {:?}: ", fruits);

    // Tuples
    let human_1: (&str, i32, bool)= ("Mary", 32, false);
    let human_2: (String, i32, bool)= ("John".to_string(), 48, true);
    println!("Human Tuple: {:?}", human_1);
    println!("Human Tuple: {:?}", human_2);

    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My  Mix Tuple: {:?}", my_mix_tuple);

    // Slices: [1, 2, 3, 4, 5]
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slice {:?}", number_slices);

    let animal_slices:&[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Anamal Slices {:?}", animal_slices);

    // Strings Vs String Slices (&str)
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {:?}", stone_cold);

    // &str (String Slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string;
    println!("Slice Value: {}", slice);
}   