fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}");
    let _reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String { // dangle returns a reference to a String
    let s = String::from("hello");
    s
} // s goes out of scope and is dropped. Its memory goes away
  // So instead of returning a reference, we return s itself
