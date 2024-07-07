use meval::{Context, Expr};
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
    let mut context = Context::new();

    for (key, &value) in variables.iter() {
        context.var(key, value);
    }

    context.func2("mod", mod_fn);
    context.func("fib", fib_fn);
    context.func("abs", abs_fn);
    context.func2("max", max_fn);
    context.func2("min", min_fn);

    let expr: Expr = expr_str.parse().map_err(|_| "Invalid expression".to_string())?;
    
    expr.eval_with_context(&context).map_err(|_| "Evaluation error".to_string())
}

fn mod_fn(a: f64, b: f64) -> f64 {
    a % b
}

fn fib_fn(n: f64) -> f64 {
    let n = n as u32;
    if n == 0 {
        return 0.0;
    } else if n == 1 {
        return 1.0;
    }

    let mut a: u32= 0;
    let mut b: u32 = 1;
    let mut c: u32;

    for _ in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }

    b as f64
}

fn abs_fn(a: f64) -> f64 {
    a.abs()
}

fn max_fn(a: f64, b: f64) -> f64 {
    a.max(b)
}

fn min_fn(a: f64, b: f64) -> f64 {
    a.min(b)
}
