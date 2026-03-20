#![allow(unused)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function that creates a new instance
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // Calculates the area of an instance
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Checks if the current instance of Rectangle fit into another instance of Rectangle
    fn can_fit(&self, other: &Rectangle) -> bool {
        self.width <= other.width && self.height <= other.height
    }
}

fn main() {
    let rect1 = Rectangle::new(10, 100);
    println!("The area of rectangle 1 is: {}", rect1.area());

    let rect2 = Rectangle::new(10, 10);
    println!("The area of rectangle 2 is: {}", rect2.area());

    let condition = rect1.can_fit(&rect2);

    if condition {
        println!("Rectangle 1 fits into rectangle 2")
    } else {
        println!("Rectangle 1 does not fit into rectangle 2")
    }
}
