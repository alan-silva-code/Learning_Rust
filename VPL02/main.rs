use std::io;

/*
Zero to Cancel
Your boss is on the phone, nervous. He wants you to calculate the sum of a sequence of numbers he's going to tell you over the phone, to find out the total sales from his most recent business trip.

Unfortunately, from time to time your boss gives you the wrong numbers on the phone.

Fortunately, your boss quickly realizes he's given a wrong number and says "zero," which, as previously agreed, means ignore the last current number.

Unfortunately, your boss may make repeated mistakes, and says "zero" for each mistake.

*/

fn vector() -> Vec<i32> {
    let v: Vec<i32>= Vec::new();
    return v;
}

fn input() -> i32{
    let mut input_number = String::new();
    io::stdin()
    .read_line(&mut input_number)
    .expect("Error! Please try again.");

    let number: i32 = input_number.trim().parse().expect("Enter a valid number!");
    return number;
}

fn method() -> Vec<i32>{    
    let mut i: i32 = 0;
    let size: i32 = input();
    let mut v = vector();
    while i != size {
        let number = input();
        if number != 0 {
            v.push(number);
            i = i + 1;
        } else if number == 0 {
            v.pop();
            i = i + 1;
        } else {
            println!("Error!");
        }
    }

    return v;
}

fn sum() -> i32{
    let v = method();

    let mut k = 0;
    let mut sum: i32 = 0;
    while k != (v.len()) {
        sum = sum + v[k];
        k = k + 1;
    }
    
    return sum;
}

fn main(){
    let _sum = sum();
    println!("{}", _sum);
}