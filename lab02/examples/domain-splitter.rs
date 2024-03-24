fn main() {
    let mut url = String::new();
    println!("Enter url:");
    std::io::stdin().read_line(&mut url).unwrap();
    let s1 = domain_splitter(&url);
    println!("{:?}", s1);
}

fn domain_splitter(url: &str) -> (String, String, Vec<&str>) {
    let mut i = 0;

    while i < url.len() && url.chars().nth(i).unwrap() != '/' {
        i += 1;
    }

    while i < url.len() && !url.chars().nth(i).unwrap().is_alphabetic() {
        i += 1;
    }
    let mut p = i;

    while i < url.len() && url.chars().nth(i).unwrap() != '/' {
        i += 1;
    }

    let k = i;

    if url[p..p + 4].to_string() == "www." {
        p += 4;
    }

    while i < url.len() && url.chars().nth(i).unwrap() != '?' {
        i += 1;
    }

    let mut dir = i;

    let mut domain = url[p..k].to_string();
    let mut path = url[k..dir].to_string();

    if i < url.len() && url.chars().nth(i).unwrap() == '?' {
        dir += 1;
    }

    let args = url[dir..].trim_end();
    let arguments: Vec<&str> = args.split('&').collect();

    if let Some('\n') = domain.chars().last() {
        domain.pop();
    }

    if let Some('\n') = path.chars().last() {
        path.pop();
    }

    (domain, path, arguments)
}
