fn main() {

    let a=[1,2,43,3,54];
    let b=[23,5];

    println!("{}",len_longer_array(&a, &b));
    let c=longer_array(&a, &b);
    println!("{:?}",c);

    let text = String::from("Introduction to a long text. The rest of long text with many sentences.");
 
    let intro = text.split('.').next().expect("Could not find a first sentence.");
 
    let i = Introduction { intro };
    println!("{:?}",i);
    i.print();

    println!("{}",i.intro_text());

    println!("{}",get_sample_text());
}
#[derive(Debug)]
struct Introduction<'a> {
    intro : &'a str
}

impl<'a> Introduction<'a> {
    fn print(&self) {
        println!("{}", self.intro);
    }

    fn intro_text(&self)->&str{
        self.intro
    }
}

fn len_longer_array(a : &[i32], b : &[i32]) -> usize {
    if a.len() > b.len() {
        a.len()
    } else {
        b.len()
    }
}

fn longer_array<'a>(a : &'a[i32], b : &'a[i32]) -> &'a[i32] {
    if a.len() > b.len() {
        &a
    } else {
        &b
    }
}

fn get_sample_text() -> &'static str {
    String::from("Just a sample text").as_str()
}