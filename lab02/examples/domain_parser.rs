fn main() {
    let mut url = String::new();
    println!("Enter url:");
    std::io::stdin().read_line(&mut url).unwrap();
    let s1 = domain_parser(&url);
    println!("{s1}");
}

pub fn domain_parser(url:&str)->String{
    let mut i=0;

    while i<url.len() && url.chars().nth(i).unwrap()!='/'{
        i+=1;
    }

    while i<url.len() && !url.chars().nth(i).unwrap().is_alphabetic(){
        i+=1;
    }
    let mut p=i;

    while i<url.len() && url.chars().nth(i).unwrap()!='/'{
        i+=1;
    }

    let k=i;

    if url[p..p+4].to_string()=="www."{
        p+=4;
    }

    url[p..k].to_string()
}
