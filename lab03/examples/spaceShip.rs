use std::f32::consts::PI;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Clone)]
struct Vec2D {
    x: f32,
    y: f32,
}

impl Vec2D {
    fn unit_vec() -> Vec2D {
        Vec2D { x: 1f32, y: 1f32 }
    }

    fn equals(&self, other: &Vec2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector2d({},{})", self.x, self.y);
        Ok(())
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

#[derive(Clone)]
struct SpaceShip {
    position: Vec2D,
    direction: f32,
}
impl SpaceShip {
    fn new(position: Vec2D) -> SpaceShip {
        SpaceShip {
            position,
            direction: 0f32,
        }
    }

    fn turn(&mut self, angle: f32) {
        let curr_deg = angle + self.direction;
        if curr_deg > 180f32 {
            self.direction = curr_deg - 180f32
        } else {
            self.direction = curr_deg
        }
    }

    fn print_position(&self) {
        println!("Current position: {}", self.position)
    }

    fn dist_point(&self, point: Vec2D) -> f32 {
        let x = self.position.x - point.x;
        let y = self.position.y - point.y;

        (x.powf(2f32) + y.powf(2f32)).sqrt()
    }

    fn dist_spaceship(&self, other: SpaceShip) -> f32 {
        let x = self.position.x - other.position.x;
        let y = self.position.y - other.position.y;

        (x.powf(2f32) + y.powf(2f32)).sqrt()
    }

    fn move_vector(&mut self, vector: Vec2D) {
        self.position = self.position.clone() + vector;
    }

    fn move_distance(&mut self, distance: f32) {
        self.position = self.position.clone()
            + Vec2D {
                x: distance * (self.direction / 180f32  * PI).cos(),
                y: distance * (self.direction / 180f32  * PI).sin(),
            }
    }
}

impl Display for SpaceShip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SpaceShip[position={},direction={}",
            self.position, self.direction
        );
        Ok(())
    }
}

fn main() {
    let mut v1 = Vec2D::unit_vec();
    let v2 = Vec2D::unit_vec();

    println!("v1: {}", v1);
    println!("v1==v2: {}", v1.equals(&v2));

    let v3 = v1.clone() + v2;
    println!("v3: {}", v3);

    v1.y = 5f32;

    let v4 = v1 - v3.clone();
    println!("v4: {}", v4);

    let mut s1 = SpaceShip::new(v3.clone());

    println!("s1: {}", s1);

    s1.print_position();

    let s2 = SpaceShip {
        position: Vec2D { x: 4f32, y: 2f32 },
        direction: 50f32,
    };

    println!("s2: {}", s2);
    println!("Dist (s1,s2): {}", s1.dist_spaceship(s2.clone()));

    println!("Dist (s1,v3):{}", s2.dist_point(v3.clone()));

    s1.turn(45f32);
    println!("{}", s1);

    s1.move_vector(v3);
    println!("{}", s1);

    s1.move_distance(2.8284f32);
    println!("{}", s1);
}
