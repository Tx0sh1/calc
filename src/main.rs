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

    let mut a = String::new();
    println!("Enter first number: ");
    std::io::stdin()
        .read_line(&mut a)
        .expect("Failed to parse input");
    let a = a.trim().parse::<i32>().expect("Failed to parse input");

    let mut b = String::new();
    println!("Enter second number: ");
    std::io::stdin()
        .read_line(&mut b)
        .expect("Failed to parse input");
    let b = b.trim().parse::<i32>().expect("Failed to parse input");

    let sum = a + b;
    println!("The sum is: {}", sum);
}

fn subtract() {

    let mut a = String::new();
    println!("Enter first number: ");
    std::io::stdin()
        .read_line(&mut a)
        .expect("Failed to parse input");
    let a = a.trim().parse::<i32>().expect("Failed to parse input");

    let mut b = String::new();
    println!("Enter second number: ");
    std::io::stdin()
        .read_line(&mut b)
        .expect("Failed to parse input");
    let b = b.trim().parse::<i32>().expect("Failed to parse input");

    let difference = a - b;
    println!("The difference is: {}", difference);
}

fn multiply() {

    let mut a = String::new();
    println!("Enter first number: ");
    std::io::stdin()
        .read_line(&mut a)
        .expect("Failed to parse input");
    let a = a.trim().parse::<i32>().expect("Failed to parse input");

    let mut b = String::new();
    println!("Enter second number: ");
    std::io::stdin()
        .read_line(&mut b)
        .expect("Failed to parse input");
    let b = b.trim().parse::<i32>().expect("Failed to parse input");

    let product = a * b;
    println!("The product is: {}", product);
}

fn divide() {

    let mut a = String::new();
    println!("Enter first number: ");
    std::io::stdin()
        .read_line(&mut a)
        .expect("Failed to parse input");
    let a = a.trim().parse::<i32>().expect("Failed to parse input");

    let mut b = String::new();
    println!("Enter second number: ");
    std::io::stdin()
        .read_line(&mut b)
        .expect("Failed to parse input");
    let b = b.trim().parse::<i32>().expect("Failed to parse input");

    if b != 0 {
        let quotient = a / b;
        println!("The quotient is: {}", quotient);
    } else {
        println!("Cannot divide by zero");
    }
}