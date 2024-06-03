use std::collections::HashMap;
use std::io::{self, Write};
use meval::eval_str;

enum Value {
    Number(f64),
    Text(String),
    NumberArray(Vec<f64>),
    TextArray(Vec<String>),
}

struct Interpreter {
    variables: HashMap<String, Value>,
    has_error: bool,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            has_error: false,
        }
    }

    fn execute(&mut self, line: &str) {
        self.has_error = false;

        if line.starts_with("gand mara") {
            std::process::exit(0);
        }

        if line.starts_with("chol ") {
            self.parse_let(line);
        } else if line.starts_with("bol ") {
            self.parse_print(line);
        } else if line.starts_with("jodi ") {
            self.parse_if(line);
        } else if line.starts_with("jotokhun ") {
            self.parse_while(line);
        } else if line.starts_with("anko ") {
            self.parse_input_number(line);
        } else if line.starts_with("shobdo ") {
            self.parse_input_string(line);
        } else {
            self.report_unknown_command(line);
        }
    }

    fn parse_let(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 && parts[2] == "=" {
            let var_name = parts[1].to_string();
            let value_expr = &parts[3..].join(" ");
            if value_expr.starts_with("[") && value_expr.ends_with("]") {
                let elements: Vec<&str> = value_expr[1..value_expr.len() - 1].split(',').map(|s| s.trim()).collect();
                if elements.iter().all(|e| e.parse::<f64>().is_ok()) {
                    let number_array: Vec<f64> = elements.iter().map(|e| e.parse::<f64>().unwrap()).collect();
                    self.variables.insert(var_name, Value::NumberArray(number_array));
                } else if elements.iter().all(|e| e.starts_with("\"") && e.ends_with("\"")) {
                    let string_array: Vec<String> = elements.iter().map(|e| e[1..e.len() - 1].to_string()).collect();
                    self.variables.insert(var_name, Value::TextArray(string_array));
                } else {
                    println!("Dada array elements ta theek na");
                }
            } else if value_expr.starts_with("\"") && value_expr.ends_with("\"") {
                let string_value = value_expr[1..value_expr.len() - 1].to_string();
                self.variables.insert(var_name, Value::Text(string_value));
            } else {
                match self.evaluate_expression(value_expr) {
                    Ok(value) => {
                        self.variables.insert(var_name, Value::Number(value));
                    }
                    Err(err) => {
                        println!("Dada Ei expression evaluate korte parchi na: {}", err);
                    }
                }
            }
        } else {
            println!("Dada chol commander jonno syntax ta theek na");
        }
    }

    fn parse_print(&self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let var_name = parts[1];
            if let Some(index) = self.parse_array_index(var_name) {
                let (base_name, idx) = index;
                match self.variables.get(base_name) {
                    Some(Value::NumberArray(values)) => {
                        if let Some(&value) = values.get(idx) {
                            println!("{}", value);
                        } else {
                            println!("Dada Ei index ta array er moddhe nei: {}", idx);
                        }
                    }
                    Some(Value::TextArray(values)) => {
                        if let Some(value) = values.get(idx) {
                            println!("{}", value);
                        } else {
                            println!("Dada Ei index ta array er moddhe nei: {}", idx);
                        }
                    }
                    _ => println!("Dada array ta defined na: {}", base_name),
                }
            } else {
                match self.variables.get(var_name) {
                    Some(Value::Number(value)) => println!("{}", value),
                    Some(Value::Text(value)) => println!("{}", value),
                    Some(Value::NumberArray(values)) => println!("{:?}", values),
                    Some(Value::TextArray(values)) => println!("{:?}", values),
                    None => println!("Dada variable ta defined na: {}", var_name),
                }
            }
        } else {
            println!("Dada bol commander jonno syntax ta theek na");
        }
    }

    fn parse_if(&self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 && parts[parts.len() - 2] == "bol" {
            let condition = &parts[1..parts.len() - 2].join(" ");
            let output = parts[parts.len() - 1];
            match self.evaluate_condition(condition) {
                Ok(true) => {
                    if let Ok(value) = output.parse::<f64>() {
                        println!("{}", value);
                    } else if output.starts_with("\"") && output.ends_with("\"") {
                        let string_value = &output[1..output.len() - 1];
                        println!("{}", string_value);
                    } else {
                        match self.variables.get(output) {
                            Some(Value::Number(value)) => println!("{}", value),
                            Some(Value::Text(value)) => println!("{}", value),
                            Some(Value::NumberArray(values)) => println!("{:?}", values),
                            Some(Value::TextArray(values)) => println!("{:?}", values),
                            None => println!("Dada Ei variable ta defined na: {}", output),
                        }
                    }
                }
                Ok(false) => {}
                Err(err) => println!("Dada Jodi command evaluate korar somay error: {}", err),
            }
        } else {
            println!("Dada Jodi command er jonnio syntax ta theek na");
        }
    }

    fn parse_while(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 && parts[parts.len() - 1] == "obdhi" {
            let condition = &parts[1..parts.len() - 1].join(" ");
            let mut body = String::new();
            loop {
                print!("| ");
                io::stdout().flush().unwrap();
                let mut body_line = String::new();
                io::stdin().read_line(&mut body_line).expect("Dada line ta pora jache na");
                if body_line.trim().is_empty() {
                    break;
                }
                body.push_str(&body_line);
            }

            while let Ok(true) = self.evaluate_condition(condition) {
                let commands: Vec<&str> = body.trim().split('\n').collect();
                for command in commands {
                    self.execute(command.trim());
                }
            }
        } else {
            println!("Jotokhun commander jonno syntax ta theek na");
        }
    }

    fn parse_input_number(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let var_name = parts[1].to_string();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Dada line porte parchi na");
            match input.trim().parse::<f64>() {
                Ok(value) => {
                    self.variables.insert(var_name, Value::Number(value));
                }
                Err(_) => println!("Dada valid number input korun"),
            }
        } else {
            println!("Dada anko commander jonno syntax ta theek na");
        }
    }

    fn parse_input_string(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let var_name = parts[1].to_string();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Dada line porte parchi na");
            let string_value = input.trim().to_string();
            self.variables.insert(var_name, Value::Text(string_value));
        } else {
            println!("Dada shobdo commander jonno syntax ta theek na");
        }
    }

    fn evaluate_expression(&self, expr: &str) -> Result<f64, meval::Error> {
        let substituted_expr = self.substitute_variables(expr);
        eval_str(&substituted_expr)
    }

    fn substitute_variables(&self, expr: &str) -> String {
        let mut substituted_expr = expr.to_string();
        for (var, value) in &self.variables {
            match value {
                Value::Number(num) => {
                    substituted_expr = substituted_expr.replace(var, &num.to_string());
                }
                Value::Text(_) => {}
                _ => {}
            }
        }
        substituted_expr
    }

    fn evaluate_condition(&self, condition: &str) -> Result<bool, String> {
        let parts: Vec<&str> = condition.split_whitespace().collect();
        if parts.len() == 3 {
            let left = self.evaluate_condition_operand(parts[0])?;
            let right = self.evaluate_condition_operand(parts[2])?;
            match parts[1] {
                "==" => Ok(left == right),
                "!=" => Ok(left != right),
                ">" => Ok(left > right),
                "<" => Ok(left < right),
                ">=" => Ok(left >= right),
                "<=" => Ok(left <= right),
                _ => Err(format!("Dada operator ta theek na: {}", parts[1])),
            }
        } else {
            Err("Dada ki korchen syntax ta theek na".to_string())
        }
    }

    fn evaluate_condition_operand(&self, operand: &str) -> Result<f64, String> {
        if let Ok(value) = operand.parse::<f64>() {
            return Ok(value);
        }

        match self.variables.get(operand) {
            Some(Value::Number(num)) => Ok(*num),
            Some(Value::Text(_)) => Err(format!("String variable '{}' numeric condition ei use kora jabe na", operand)),
            Some(Value::NumberArray(_)) => Err(format!("Array element '{}' numeric condition ei use kora jabe na", operand)),
            Some(Value::TextArray(_)) => Err(format!("Array element '{}' numeric condition ei use kora jabe na", operand)),
            None => Err(format!("Dada ei variable ta jaani na '{}'", operand)),
        }
    }

    fn parse_array_index<'a>(&'a self, var_name: &'a str) -> Option<(&'a str, usize)> {
        if let Some(open_bracket) = var_name.find('[') {
            if let Some(close_bracket) = var_name.find(']') {
                let base_name = &var_name[0..open_bracket];
                if let Ok(index) = var_name[open_bracket + 1..close_bracket].parse::<usize>() {
                    return Some((base_name, index));
                }
            }
        }
        None
    }

    fn report_unknown_command(&mut self, line: &str) {
        if !self.has_error {
            println!("Dada Ei command ta jaani na: {}", line);
            self.has_error = true;
        }
    }
}

fn main() {
    let mut interpreter = Interpreter::new();
    let mut input = String::new();

    println!("Bong compiler ei apnar Sagoto. Shuru kora jak:");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).expect("Dada line porte parchi na");

        let trimmed = input.trim();

        interpreter.execute(trimmed);
    }
}

