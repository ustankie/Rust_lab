use rand::Rng;

fn main() {
    let s1 = password_generator(20);
    println!("{s1}");
}

fn password_generator(n: u32) -> String {
    let mut rng = rand::thread_rng();

    let mut s1 = String::from("");
    for _ in 0..n {
        let n1: u8 = rng.gen_range('!' as u8..'}' as u8);
        let cn1 = n1 as char;
        s1 = s1.to_string() + &cn1.to_string();
    }

    s1
}
