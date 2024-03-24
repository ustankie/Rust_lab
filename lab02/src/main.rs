fn main() {
    let array = [1, 2, 3, 4, 5];
    let array_slice : &[i32] = &array[2..4];
    println!("{:?}", array_slice);
}   
