fn main() {
    let mut function = String::new();
    println!("Enter the function name (add, subtract, multiply, divide or exit): ");
    std::io::stdin()
        .read_line(&mut function)
        .expect("Failed to read line");
    let function = function.trim();

    if function == "add" {
        add();
    } else if function == "subtract" {
        subtract();
    } else if function == "multiply" {
        multiply();
    } else if function == "divide" {
        divide();
    } else if function == "exit" {
        println!("Exiting...");
        return;
    }
    else {
        println!("Function not found");
    }
}

fn add() {

    let a = read_number("Enter first number: ");
    let b = read_number("Enter second number: ");


    let sum = a + b;
    println!("The sum is: {}", sum);
}

fn subtract() {

    let a = read_number("Enter first number: ");
    let b = read_number("Enter second number: ");

    let difference = a - b;
    println!("The difference is: {}", difference);
}

fn multiply() {

    let a = read_number("Enter first number: ");
    let b = read_number("Enter second number: ");

    let product = a * b;
    println!("The product is: {}", product);
}

fn divide() {

    let a = read_number("Enter first number: ");
    let b = read_number("Enter second number: ");

    if b != 0 {
        let quotient = a / b;
        println!("The quotient is: {}", quotient);
    } else {
        println!("Cannot divide by zero");
    }
}

fn read_number(prompt: &str) -> i32 {
    let mut input = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse::<i32>().expect("Failed to parse number")
}