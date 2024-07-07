use meval::Expr;
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut variables: HashMap<String, f64> = HashMap::new();
    let stdin = io::stdin();
    
    loop {
        print!("-> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        
        let input = input.trim();
        if input.is_empty() {
            println!("Goodbye!");
            std::process::exit(0);
        }
        
        match handle_input(input, &mut variables) {
            Ok(result) => println!("result: {}", result),
            Err(e) => println!("error: {}", e),
        }
    }
}

fn handle_input(input: &str, variables: &mut HashMap<String, f64>) -> Result<String, String> {
    let parts: Vec<&str> = input.split('=').collect();
    
    if parts.len() == 2 {
        let var_name = parts[0].trim();
        let expr_str = parts[1].trim();
        
        match evaluate_expression(expr_str, variables) {
            Ok(result) => {
                variables.insert(var_name.to_string(), result);
                Ok(result.to_string())
            },
            Err(e) => Err(e),
        }
    } else {
        evaluate_expression(input, variables).map(|result| result.to_string())
    }
}

fn evaluate_expression(expr_str: &str, variables: &HashMap<String, f64>) -> Result<f64, String> {
    let mut context = meval::Context::new();

    for (key, &value) in variables.iter() {
        context.var(key, value);
    }

    let expr: Expr = expr_str.parse().map_err(|_| "Invalid expression".to_string())?;
    
    expr.eval_with_context(&context).map_err(|_| "Evaluation error".to_string())
}
