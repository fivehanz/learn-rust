#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // :? indicates to Display with Debug trait;
    // :#? stylizes the Display for readability.
    println!("rect1 is {:#?}", rect1);

    // sbg! uses stderr instead of stdout;
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

impl Rectangle {
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }
}
