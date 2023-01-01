fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six is {:#?}, none is {:#?}", six, none);
    print_only_fives(Some(5));
    print_only_fives(Some(6));
    print_only_fives(Some(4));
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn print_only_fives(x: Option<u8>) {
    match x {
        Some(5) => println!("FIVE"),
        _ => () // default 
    }
}