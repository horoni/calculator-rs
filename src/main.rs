use std::io::{self, Write};

fn get_precedence(op: &str) -> isize {
    match op {
        "!" => 5,
        "sqrt" | "sin" | "cos" => 4,
        "^" => 3,
        "*" | "/" => 2,
        "+" | "-" => 1,
        _ => 0,
    }
}

fn is_postfix(op: &str) -> bool {
    match op {
        "!" => true,
        _ => false,
    }
}

fn is_prefix(op: &str) -> bool {
    match op {
        "sqrt" | "sin" | "cos" => true,
        _ => false,
    }
}

fn rpn_from_infix(exp: String) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    let mut num = String::new();
    let mut op = String::new();

    for c in exp.chars() {
        if c.is_whitespace() {
            continue;
        }
        if c.is_digit(10) || c == '.' {
            num.push(c);
            continue;
        } else {
            if !num.is_empty() {
                out.push(num.clone());
                num.clear();
            }
            if c.is_alphabetic() {
                op.push(c);
            }
            if is_postfix(c.to_string().as_str()) {
                out.push(c.to_string());
            } else if is_prefix(op.as_str()) {
                stack.push(op.clone());
                op.clear();
            } else if c == '(' {
                stack.push(c.to_string());
            } else if c == ')' {
                while let Some(remain_op) = stack.pop() {
                    if remain_op.as_str() == "(" {
                        break;
                    }
                    out.push(remain_op);
                }
            } else {
                if c.is_alphabetic() {
                    continue;
                }
                while let Some(tok) = stack.last() {
                    if is_prefix(tok.as_str())
                        || get_precedence(tok.as_str()) >= get_precedence(c.to_string().as_str())
                    {
                        out.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(c.to_string());
            }
        }
    }

    if !num.is_empty() {
        out.push(num);
    }

    while let Some(remain_op) = stack.pop() {
        out.push(remain_op.to_string());
    }

    out
}

fn factorial(n: usize) -> Option<usize> {
    if n > 20 {
        return None;
    }
    if n == 0 || n == 1 {
        return Some(1);
    }
    Some(n * factorial(n - 1).unwrap())
}

fn rpn_evaluate(exp: Vec<String>) -> Result<f64, String> {
    let mut stack = Vec::new();

    for tok in exp {
        if let Ok(num) = tok.parse::<f64>() {
            stack.push(num);
        } else {
            let mut r: f64 = 0.0;
            if !is_prefix(tok.clone().as_str()) && !is_postfix(tok.clone().as_str()) {
                r = stack.pop().ok_or("Not enough operands")?;
            }
            let l = stack.pop().ok_or("Not enough operands")?;

            match tok.as_str() {
                "+" => stack.push(l + r),
                "-" => stack.push(l - r),
                "*" => stack.push(l * r),
                "/" => stack.push(l / r),
                "^" => stack.push(l.powf(r)),
                "sqrt" => stack.push(l.sqrt()),
                "sin" => stack.push(l.to_radians().sin()),
                "cos" => stack.push(l.to_radians().cos()),
                "!" => stack.push(factorial(l.round() as usize).ok_or("factorial error")? as f64),
                _ => return Err(format!("Unknown operator {}", tok)),
            }
        }
    }

    // implicit multiplication
    while stack.len() > 1 {
        let r = stack.pop().ok_or("Not enough operands when finishing")?;
        let l = stack.pop().ok_or("Not enough operands when finishing")?;
        stack.push(l * r);
    }

    stack.pop().ok_or("Evaluate error".to_string())
}

fn rpn_from_infix_proxy(exp: String) -> Vec<String> {
    let out = rpn_from_infix(exp);
    println!("{:#?}", out);
    out
}

fn read_input(prompt: &str) -> io::Result<String> {
    let mut out = String::new();
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut out)?;
    Ok(out)
}

fn main() -> io::Result<()> {
    loop {
        let exp: String = read_input(">>> ")?;
        match exp.trim() {
            "exit" | "q" => break,
            _ => match rpn_evaluate(rpn_from_infix_proxy(exp)) {
                Ok(result) => println!("<<< {:?}", result),
                Err(e) => println!("<<< {}", e),
            },
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_ops() {
        let r_add = String::from("7+2");
        let r_sub = String::from("1-3");
        let r_mul = String::from("3*5");
        let r_div = String::from("21/7");

        assert_eq!(
            rpn_evaluate(rpn_from_infix(r_add)).unwrap(),
            9.0,
            "Add failed"
        );
        assert_eq!(
            rpn_evaluate(rpn_from_infix(r_sub)).unwrap(),
            -2.0,
            "Sub failed"
        );
        assert_eq!(
            rpn_evaluate(rpn_from_infix(r_mul)).unwrap(),
            15.0,
            "Mul failed"
        );
        assert_eq!(
            rpn_evaluate(rpn_from_infix(r_div)).unwrap(),
            3.0,
            "Div failed"
        );
    }

    #[test]
    fn translating() {
        let r_1 = String::from("3 + 4");
        let r_2 = String::from("(1 + 2) * 4 + 3");
        let r_3 = String::from("3 + 4 * 2 / (1 - 5)^2");

        assert_eq!(rpn_from_infix(r_1), vec!["3", "4", "+"]);
        assert_eq!(rpn_from_infix(r_2), vec!["1", "2", "+", "4", "*", "3", "+"]);
        assert_eq!(
            rpn_from_infix(r_3),
            vec!["3", "4", "2", "*", "1", "5", "-", "2", "^", "/", "+"]
        );
    }

    #[test]
    fn some_expressions() {
        let r_1 = rpn_from_infix(String::from("3 + 4"));
        let r_2 = rpn_from_infix(String::from("(1 + 2) * 4 + 3"));
        let r_3 = rpn_from_infix(String::from("3 + 4 * 2 / (1 - 5)^2"));

        assert_eq!(rpn_evaluate(r_1).unwrap(), 7.0);
        assert_eq!(rpn_evaluate(r_2).unwrap(), 15.0);
        assert_eq!(rpn_evaluate(r_3).unwrap(), 3.5);
    }

    #[test]
    fn sqrt() {
        let r_1 = rpn_from_infix(String::from("sqrt 25"));
        let r_2 = rpn_from_infix(String::from("sqrt (1 + 3)"));
        let r_3 = rpn_from_infix(String::from("sqrt49"));
        let r_4 = rpn_from_infix(String::from("sqrt49^2"));
        let r_5 = rpn_from_infix(String::from("3 + sqrt49^2"));

        assert_eq!(rpn_evaluate(r_1).unwrap(), 5.0);
        assert_eq!(rpn_evaluate(r_2).unwrap(), 2.0);
        assert_eq!(rpn_evaluate(r_3).unwrap(), 7.0);
        assert_eq!(rpn_evaluate(r_4).unwrap(), 49.0);
        assert_eq!(rpn_evaluate(r_5).unwrap(), 52.0);
    }

    #[test]
    fn sin_cos() {
        let r_1 = rpn_from_infix(String::from("sin 90"));
        let r_2 = rpn_from_infix(String::from("cos 180"));

        assert_eq!(rpn_evaluate(r_1).unwrap(), 1.0, "sin failed");
        assert_eq!(rpn_evaluate(r_2).unwrap(), -1.0, "cos failed");
    }
}
