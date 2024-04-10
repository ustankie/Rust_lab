use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Clone)]
struct Vec2D {
    x: f32,
    y: f32,
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector2d ({},{})", self.x, self.y)?;
        Ok(())
    }
}

impl Vec2D {
    fn unit_vec() -> Vec2D {
        Vec2D { x: 1f32, y: 1f32 }
    }

    fn equals(&self, scnd: &Vec2D) -> bool {
        (self.x == scnd.x) && (self.y == scnd.y)
    }
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, other: Vec2D) -> Self {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, other: Vec2D) -> Self {
        Vec2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn main() {
    let mut vec2d = Vec2D { x: 1f32, y: 2f32 };
    println!("{}", vec2d);

    let mut unit_vec = Vec2D::unit_vec();
    println!("{}", unit_vec);

    vec2d.y = 1f32;

    println!("{}", vec2d.equals(&unit_vec));

    unit_vec.x = 3f32;
    vec2d.y = 5f32;

    println!("{}", vec2d.clone().sub(unit_vec.clone()));
    println!("{}", vec2d.add(unit_vec));
}
