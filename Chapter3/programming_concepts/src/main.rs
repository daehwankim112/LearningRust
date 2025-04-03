use std::io;

fn main() {

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>());
    }

    let x = 5;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    let x = 6;
    println!("The value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len();

    println!("print spaces: {spaces}");

    let _guess: u32 = "42".parse().expect("Not a number!");

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
    
    // Numeric operations
    let _sum = 5 + 10;

    let _difference = 95.5 - 4.3;

    let _product = 4 * 30;

    let _quotient = 56.7 / 32.3;
    let _truncated = -5 / 3;

    let _remainder = 43 % 5;

    println!("three hours in seconds is {THREE_HOURS_IN_SECONDS}. Check if this removes the error!");

    let _t = true;
    let _f: bool = false; // with explicit type annotation

    let _c = 'z';
    let _str0 = "string maybe";
    // let _str1 = 'string'; This cause a compiler error because '' is for char
    let _z: char = 'Z';
    let _heart_eyed_cat = '😻';

    print_type_of(&_c);
    print_type_of(&_str0);
    
    // tuples
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let _tup = (500, 6.4, 1);
    let (_x, y, _z) = _tup;

    println!("The value of y is {y}");

    let _x: (i32, f64, u8) = (500, 6.4, 1);
    // println!("_x: {_x}"); Tuples cannot be printed like this.
    println!("_tuple of _tuples _x: {:?}", _x);
    let five_hundred = _x.0;
    println!("five_hundred: {five_hundred}");
    let six_point_four = _x.1;
    println!("six_point_four: {six_point_four}");
    let one = _x.2;
    println!("one: {one}");
    
    // array
    let _a = [0, 1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let _a: [i32; 6] = [0, 1, 2, 3, 4, 5];
    let _b = [3; 5]; // this is equal to b = [3, 3, 3, 3, 3];

    let _first = _a[0];
    let _second = _a[1];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index:usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = _a[index];

    println!("The value of the element at index {index} is: {element}");
}
