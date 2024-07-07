use meval::{Context, Expr};
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut variaveis: HashMap<String, f64> = HashMap::new();
    let entrada = io::stdin();
    
    loop {
        print!("-> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        entrada.read_line(&mut input).unwrap();
        
        let input = input.trim();
        if input.is_empty() {
            println!("Adeus!");
            std::process::exit(0);
        }
        
        match tratar_input(input, &mut variaveis) {
            Ok(resultado) => println!("resultado: {}", resultado),
            Err(e) => println!("erro: {}", e),
        }
    }
}

fn tratar_input(input: &str, variaveis: &mut HashMap<String, f64>) -> Result<String, String> {
    let partes: Vec<&str> = input.split('=').collect();
    
    if partes.len() == 2 {
        let nome_var = partes[0].trim();
        let expr_str = partes[1].trim();
        
        match avaliar_expressao(expr_str, variaveis) {
            Ok(resultado) => {
                variaveis.insert(nome_var.to_string(), resultado);
                Ok(resultado.to_string())
            },
            Err(e) => Err(e),
        }
    } else {
        avaliar_expressao(input, variaveis).map(|resultado| resultado.to_string())
    }
}

fn avaliar_expressao(expr_str: &str, variaveis: &HashMap<String, f64>) -> Result<f64, String> {
    let mut contexto = Context::new();

    for (chave, &valor) in variaveis.iter() {
        contexto.var(chave, valor);
    }

    contexto.func2("mod", mod_fn);
    contexto.func("fib", fib_fn);
    contexto.func("abs", abs_fn);
    contexto.func2("max", max_fn);
    contexto.func2("min", min_fn);

    let expr: Expr = expr_str.parse().map_err(|_| "Expressão inválida".to_string())?;
    
    expr.eval_with_context(&contexto).map_err(|_| "Erro de avaliação".to_string())
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

    let mut a: u32 = 0;
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
