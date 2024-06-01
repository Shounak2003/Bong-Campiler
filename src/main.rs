use std::collections::HashMap;
use std::io::{self, Write};
use meval::eval_str;

struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    fn execute(&mut self, line: &str) {
        if line.starts_with("chol ") {
            self.parse_let(line);
        } else if line.starts_with("bolun ") {
            self.parse_print(line);
        } else {
            println!("Ei command ta jani na: {}", line);
        }
    }

    fn parse_let(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 && parts[2] == "shoman"|| parts.len() >= 3 && parts[2] =="=" {
            let var_name = parts[1].to_string();
            let value_expr = &parts[3..].join(" ");
            match self.evaluate_expression(value_expr) {
                Ok(value) => {
                    self.variables.insert(var_name, value);
                }
                Err(err) => {
                    println!("Ei expression ta evaluate korchi: {}", err);
                }
            }
        } else {
            println!("let commander jonno expression ta invalid");
        }
    }

    fn parse_print(&self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let var_name = parts[1];
            match self.variables.get(var_name) {
                Some(value) => println!("{}", value),
                None => println!("dada variable ta Undefined: {}", var_name),
            }
        } else {
            println!("Print commander jonno syntax ta inavlid");
        }
    }

    fn evaluate_expression(&self, expr: &str) -> Result<f64, meval::Error> {
        eval_str(expr)
    }
}

fn main() {
    let mut interpreter = Interpreter::new();
    let mut input = String::new();

    println!("Rust Interpreter ei sagoto.Commands likhun:");

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure prompt is printed

        input.clear();
        io::stdin().read_line(&mut input).expect("Ei line ta porte parchi na");

        let trimmed = input.trim();

        if trimmed == "bye" {
            break;
        }

        interpreter.execute(trimmed);
    }

    println!("Gand mara!");
}
