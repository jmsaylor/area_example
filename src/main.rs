fn main() {
    let rect1 = Rectangle {
        width: 18,
        length: 80,
    };
    let rect2 = Rectangle {
        width: 8,
        length: 800,
    };
    let rect3 = Rectangle {
        width: 14,
        length: 2,
    };

    // println!("{:?}", rect);
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rect.area()
    // );
    println!("Can rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 can hold rect3? {}", rect1.can_hold(&rect3));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.length > rect.length
    }
}

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
