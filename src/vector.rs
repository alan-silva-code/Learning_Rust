fn main(){
    /*
    let _v:  Vec<i32> = Vec::new();
    let _v: Vec<i32> = vec![1, 2, 3];

    let mut _v:Vec<i32> = Vec::new();

    _v.push(5);
    _v.push(7);
    _v.push(9);

    println!("{:?}", _v);
    */

    let _v: Vec<f32> = vec![12.34, 18.98, 7.98, 13.249];

    // let third: &f32 = &_v[2];
    // println!("The third element is {third}");

    let third: Option<&f32> =_v.get(2);
    match third {
        Some(_) => println!("The third element is {:?}", third),
        None => println!("There is no third element"),
    }

}