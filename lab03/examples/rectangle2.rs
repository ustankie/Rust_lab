use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Rectangle {
    x: f32,
    y: f32,
}
impl Rectangle {
    fn area(&self) -> f32 {
        println!("Rectangle!");
        self.x * self.y
    }

    fn perimeter(&self) -> f32 {
        2f32 * self.x + 2f32 * self.y
    }

    fn scale(&mut self, factor: f32) {
        self.x = self.x * factor;
        self.y *= factor;
    }

    fn new_square(x: f32) -> Rectangle {
        Rectangle { x, y: x }
    }
}

trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;

    fn describe(&self) {
        println!("I'm a general shape.");
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        println!("Shape!");
        self.x * self.y
    }

    fn perimeter(&self) -> f32 {
        2f32 * self.x + 2f32 * self.y
    }

    fn describe(&self) {
        println!("I'm a rectangle.");
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle[x={},y={}]", self.x, self.y);
        Ok(())
    }
}

fn main() {
    let mut r = Rectangle { x: 1.0, y: 3.0 };

    println!("{:?}", r);
    println!("{},{}", r.x, r.y);
    let r2 = Rectangle { x: 4.0, ..r };
    println!("{:?}", r2);
    r.x = 2.0;
    println!("{:?}", r);

    println!(
        "Area of {:?} is {}, Circumf. is {}",
        r,
        r.area(),
        r.perimeter()
    );

    r.scale(2.0);
    println!("scaled {:?}", r);

    let square = Rectangle::new_square(5.0);
    println!("square: {:?}", square);

    println!("{:?}", r.describe());

    println!("{}", r);
}
