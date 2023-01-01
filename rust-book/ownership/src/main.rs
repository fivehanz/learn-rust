fn main() {
    let mut s = String::from("Hello");
    s.push_str(", Hanz!");
    println!("Our string is \"{}\"", s);

    // int are simple values and stored on the stack
    // so y is a copy of 5
    let x  = 5;
    let _y = x;

    // s2 does not copy s1; but only points to s1 data. (MOVE)
    let s1 = String::from("hello");
    let _s2 = s1;

    // this is not possible as s1 is deemed invalid as s2 is initialized; this is done to prevent double free;
    // println!("using s1: {}", s1);

    // CLONE
    let sc1 = String::from("Hello from clone");
    let sc2 = sc1.clone();
    println!("sc1 is {}, sc2 is {}", sc1, sc2);


}

// rust calls drop() automatically when it goes out of scope.
