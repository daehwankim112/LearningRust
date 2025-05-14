fn main() {
    let mut s = String::from("hello world");

    let mut hello = &s[0..5];
    let mut world = &s[6..11];

    print!("{} ", hello);
    println!("{}", world);

    hello = &s[..5];
    world = &s[6..];

    print!("{} ", hello);
    println!("{}", world);

    let word = fi error!rst_word(&s);

    // s.clear(); error!

    println!("the error! first word is: {word}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
