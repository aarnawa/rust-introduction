use std::io; // this is how you import a library

const OPS: &str = "+-*/";
fn main() {
    println!("This is a calculator");

    loop {
        println!("Enter your expression");

        // Get input from stdio
        let mut expression = String::new();
        io::stdin()
            .read_line(&mut expression)
            .expect("Failed to read line"); // if error is found, it will produce this message
        expression.retain(|c| !c.is_whitespace());
        expression = expression.trim().to_string();
        if !is_valid_expression(&expression) {
            println!("Your expression is not in the right format!\nMake sure there are only numerical values and one operator.\n");
            continue;
        }
        // println!("Your input was: {expression}");
        // find position of operator then split (O(n))
        // then check if either side in numeric
        let mut index: isize = -1;
        match expression.find(|c: char| OPS.contains(c)) {
            Some(i) => index = i as isize,
            None => println!("Your expession does not contain an operator!"),
        }
        if index == -1 {
            continue;
        }

        let (left, right) = expression.split_at(index as usize);
        let operator: char = right.chars().nth(0).expect("no operator");
        let right = &right[1..]; // now right is just the number after the operator
        // println!("operands: {left}, {right}");
        // println!("operator: {operator}");
        let operand1: isize = left.trim().parse().unwrap();
        let operand2: isize = right.trim().parse().unwrap();

        let result = perform_operation(operator, operand1, operand2).unwrap();
        println!("result: {result}");

    }
}

fn is_valid_expression(str_to_check: &str) -> bool {

    let is_valid = str_to_check.chars().all(|c| 
        {c.is_numeric() || OPS.contains(c)
    });

    return is_valid;
}

fn perform_operation(operator: char, operand_1: isize, operand_2: isize) -> Result<isize, String> {
    match operator {
        '+' => Ok(operand_1 + operand_2),
        '-' => Ok(operand_1 - operand_2),
        '*' => Ok(operand_1 * operand_2),
        '/' => {
            if operand_2 == 0 {
                Err("Division by zero".to_string())
            } else {
                Ok(operand_1 / operand_2)
            }
        },
        _ => Err("Invalid operator".to_string())
    }
}