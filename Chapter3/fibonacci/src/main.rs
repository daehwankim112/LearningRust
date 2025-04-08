fn main() {
    // Print Fibonacci number
    let mut number1 = 0;
    let mut number2 = 1;

    for i in 1..10 {
        if i == 1 {
            println!("number_sum: {number1}");
            println!("number_sum: {number2}");
        }

        let number_sum = number1 + number2;
        number1 = number2;
        number2 = number_sum;
        println!("number_sum: {number_sum}");
    }
}
