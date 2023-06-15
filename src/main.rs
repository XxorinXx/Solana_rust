use std::io;

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(operator: Operator, operand1: f64, operand2: f64) -> Option<f64> {
    match operator {
        Operator::Add => Some(operand1 + operand2),
        Operator::Subtract => Some(operand1 - operand2),
        Operator::Multiply => Some(operand1 * operand2),
        Operator::Divide => {
            if operand2 != 0.0 {
                Some(operand1 / operand2)
            } else {
                None // Division by zero error
            }
        }
    }
}

fn get_operator() -> Operator {
    loop {
        println!("Select operator:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => break Operator::Add,
            "2" => break Operator::Subtract,
            "3" => break Operator::Multiply,
            "4" => break Operator::Divide,
            _ => println!("Invalid operator. Please try again."),
        }
    }
}

fn get_number() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(number) => break number,
            Err(_) => println!("Invalid number. Please try again."),
        }
    }
}

fn main() {
    loop {
        println!("Simple Calculator");
        let operator = get_operator();

        println!("Enter the first number:");
        let operand1 = get_number();

        println!("Enter the second number:");
        let operand2 = get_number();

        if let Some(result) = calculate(operator, operand1, operand2) {
            println!("Result: {}", result);
        } else {
            println!("Error: Division by zero");
        }

        println!("1. Perform another calculation");
        println!("2. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim() {
            "1" => continue,
            "2" => break,
            _ => println!("Invalid option. Exiting..."),
        }
    }
}
