use std::io;
use std::io::Write;

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn from_str(input: &str) -> Result<Operation, String> {
        match input.trim() {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "*" => Ok(Operation::Multiply),
            "/" => Ok(Operation::Divide),
            _ => Err(format!("Невідома операція: {}", input)),
        }
    }

    fn apply(&self, a: f64, b: f64) -> Result<f64, String> {
        match *self {
            Operation::Add => Ok(a + b),
            Operation::Subtract => Ok(a - b),
            Operation::Multiply => Ok(a * b),
            Operation::Divide => {
                if b == 0.0 {
                    Err("Ділення на нуль!".to_string())
                } else {
                    Ok(a / b)
                }
            }
        }
    }
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Не вдалося прочитати введення");
    input.trim().to_string()
}

fn parse_number(input: &str) -> Result<f64, String> {
    input
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("Невірний формат числа: {}", input))
}

fn evaluate_polish_notation(expression: &str) -> Result<f64, String> {
    let tokens: Vec<&str> = expression.trim().split_whitespace().collect();
    if tokens.len() != 3 {
        return Err(
            "Неправильний формат виразу. Використовуйте: <операція> <число1> <число2>".to_string(),
        );
    }

    let operation = Operation::from_str(tokens[0])?;
    let a = parse_number(tokens[1])?;
    let b = parse_number(tokens[2])?;

    operation.apply(a, b)
}

fn main() {
    let mut memory: f64 = 0.0;

    loop {
        println!("\nПоточний результат: {}", memory);

        let mode_input = read_input("Оберіть режим (1 - звичайний, 2 - польська нотація): ");
        if mode_input == "2" {
            let expression =
                read_input("Введіть вираз у польській нотації (<операція> <число1> <число2>): ");
            match evaluate_polish_notation(&expression) {
                Ok(result) => {
                    println!("Результат: {}", result);
                    memory = result;
                }
                Err(e) => {
                    println!("Помилка: {}", e);
                    continue;
                }
            }
        } else {
            let first_input =
                read_input("Введіть перше число (або 'm' для використання результату з пам'яті): ");
            let a = if first_input.trim() == "m" {
                memory
            } else {
                match parse_number(&first_input) {
                    Ok(num) => num,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            };

            let operation_input = read_input("Введіть операцію (+, -, *, /): ");
            let operation = match Operation::from_str(&operation_input) {
                Ok(op) => op,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            let second_input = read_input("Введіть друге число: ");
            let b = match parse_number(&second_input) {
                Ok(num) => num,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            match operation.apply(a, b) {
                Ok(result) => {
                    println!("Результат: {}", result);
                    memory = result;
                }
                Err(e) => {
                    println!("Помилка: {}", e);
                }
            }
        }

        let exit_input = read_input("Бажаєте продовжити? (так/ні): ");
        if exit_input.trim().to_lowercase() == "ні" {
            break;
        }
    }
}
