use std::fmt::{Display, Formatter};

fn main() {
    let r = Rectangle { x: 1.0, y: 2.0 };
    println! {"{:?}",r};

    println!("x: {}, y: {}", r.x, r.y);

    let mut r2 = Rectangle { x: 3.0, ..r };
    println! {"{:?}", r2};
    println!("x: {}, y: {}", r2.x, r2.y);

    r2.y = 9.0;
    println! {"{:?}", r2};

    println!("Area of {:?} is {}", r2, r2.area());
    println!("Circumference of {:?} is {}", r2, r2.circumference());

    r2.scale(2.0);
    println!("Area of {:?} is {}", r2, r2.area());

    let mut r = Rectangle { x: 5.0, y: 4.0 };
    r.scale(2.0);
    println!("Area of r is {}", r.area());

    println!();
    let square = Rectangle::new_square(5.0);
    println!("Square: {}", square);

    r2.describe();
}

#[derive(Debug)]

struct Rectangle {
    x: f32,
    y: f32,
}
trait Shape {
    fn area(&self) -> f32;
    fn circumference(&self) -> f32;

    fn describe(&self) {
        println!("I'm a general shape.");
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        println!("Shape!");
        self.x * self.y
    }

    fn circumference(&self) -> f32 {
        println!("Shape!");
        2.0 * (self.x + self.y)
    }

    fn describe(&self) {
        println!("I'm a rectangle.");
    }
}

impl Rectangle {
    fn area(&self) -> f32 {
        println!("rec!");
        self.x * self.y
    }

    fn circumference(&self) -> f32 {
        println!("rec!");
        2.0 * (self.x + self.y)
    }

    fn scale(&mut self, factor: f32) {
        self.x = self.x * factor;
        self.y = self.y * factor;
    }

    fn new_square(x: f32) -> Rectangle {
        Rectangle { x, y: x }
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle[x={},y={}]", self.x, self.y)?;
        Ok(())
    }
}
