fn trim_me(input: &str) -> String {
    input.trim().to_string()
    
}

fn trim_me2(input: &str) -> String {
    let mut p=0;
    let mut k=input.len()-1;

    while p<input.len() && input.chars().nth(p).unwrap()==' '{
        p+=1;
    }

    while k>0 && input.chars().nth(k).unwrap()==' '{
        k-=1;
    }

    

    input[p..k+1].to_string()
}
fn compose_me(input: &str) -> String {
    input.to_string()+" world!"
}
 
fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}
 
fn main() {
    assert_eq!(trim_me2("Hello!     "), "Hello!");
    assert_eq!(trim_me2("  What's up!"), "What's up!");
    assert_eq!(trim_me2("   Hola!  "), "Hola!");

 
 
    assert_eq!(compose_me("Hello"), "Hello world!");
    assert_eq!(compose_me("Goodbye"), "Goodbye world!");
 
 
    assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
    assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
}