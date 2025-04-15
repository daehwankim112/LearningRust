fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    
    println!("s1 = {s1}, s2 = {s2}");

    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
    
    // This will not work because of the concept of ownership
    // let s3 = String::from("hello");
    // let s4 = s3;

    // println!("s4 = {s4}, s3 = {s3}");

    let s = String::from("hello");
    
    // s's value moves into the function...
    // ... and so is no longer valid here
    takes_ownership(s);

    let x = 5;
    // because i32 implements the Copy trait
    // x does not move into the function
    // so it's okay to use x afterword
    
    makes_copy(x);

    println!("{}", x);

    let s6 = String::from("hello");
    let s5 = gives_ownership();
    let s7 = takes_and_gives_back(s6);
}

fn takes_ownership(some_string: String) { // some string comes into scope
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
