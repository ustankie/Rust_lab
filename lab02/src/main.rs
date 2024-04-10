fn main() {
    let mut s:&str = ("text that illustrates slices");
    let first = first_word(&s[..]);
    
    // s.clear(); //tries to modify text
 
    println!("{first}");
}   
 
fn first_word(s : &str) -> &str {
    let bytes = s.as_bytes();
 
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    s
}