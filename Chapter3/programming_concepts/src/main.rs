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

    // functions

    another_function(5);
    print_labeled_measurement(10, 'h');

    // Statements are instructions that perform some action and do not return a value
    // Expressions evaluate to a resultant value. 
    // for example of a statement
    let _y = 6;
    // Statements do not return values. This will cause an error.
    // let x = (let y = 6);
    // Calling a function is an expression.
    
    let _y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {_y}");

    let _x = five();

    println!("The value of x is {_x}");

    let _x = plus_one(_x);

    println!("The value of x after plus one is {_x}");

    let number = 3;

    if number < 5 {
        println!("condition are true");
    } else {
        println!("condition was false");
    }

    let _condition = true;

    // This won't work. Variables must have a single type
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");

    // Different types of loop

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let _a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", _a[index]);

        index += 1;
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
    // This works because 5 itself does not have semicolon at the end which mean it is an
    // expression.
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
