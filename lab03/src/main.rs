struct EmptyStrucrture;
struct Color(i32, i32, i32);
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    let es = EmptyStrucrture;
    let black = Color(0, 0, 0);
    let point = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    println!("{}", black.0);
}
