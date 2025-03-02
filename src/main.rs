use std::io::{self, Write};

fn get_precedence(op: &str) -> isize {
    match op {
        "!" => 5,
        "sqrt" | "sin" | "cos" | "tan" | "cot" | "sec" | "csc" | "arcsin" | "arccos" | "arctan"
        | "arccot" | "arcsec" | "arccsc" => 4,
        "^" => 3,
        "*" | "/" => 2,
        "+" | "-" => 1,
        _ => 0,
    }
}

fn is_binary(op: &str) -> bool {
    match op {
        "+" | "-" | "*" | "/" | "^" => true,
        _ => false,
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
        "sqrt" | "sin" | "cos" | "tan" | "cot" | "sec" | "csc" | "arcsin" | "arccos" | "arctan"
        | "arccot" | "arcsec" | "arccsc" => true,
        _ => false,
    }
}

fn rpn_from_infix(exp: String) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    let mut num = String::new();
    let mut op = String::new();
    let mut last_op = String::from("Null");

    for c in exp.chars() {
        if c.is_whitespace() {
            continue;
        }
        if c == '-' {
            if last_op == "Null"
                || last_op == "("
                || is_binary(last_op.as_str())
                || is_prefix(last_op.as_str())
            {
                num.push(c);
                continue;
            }
        }
        last_op = c.to_string();
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
                last_op = op.clone();
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

fn round_close(value: f64) -> f64 {
    if (value - value.round()).abs() < 1e-5 {
        value.round()
    } else {
        value
    }
}

fn arccot(angle: f64) -> f64 {
    (angle / (1.0 + angle.powf(2.0)).sqrt()).acos()
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
                "sin" => stack.push(round_close(l.to_radians().sin())),
                "cos" => stack.push(round_close(l.to_radians().cos())),
                "tan" => stack.push(round_close(l.to_radians().tan())),
                "cot" => stack.push(round_close(l.to_radians().cos() / l.to_radians().sin())),
                "sec" => stack.push(round_close(1.0 / l.to_radians().cos())),
                "csc" => stack.push(round_close(1.0 / l.to_radians().sin())),
                "arcsin" => stack.push(round_close(l.asin().to_degrees())),
                "arccos" => stack.push(round_close(l.acos().to_degrees())),
                "arctan" => stack.push(round_close(l.atan().to_degrees())),
                "arccot" => stack.push(round_close(arccot(l).to_degrees())),
                "arcsec" => stack.push(round_close((1.0 / l).acos().to_degrees())),
                "arccsc" => stack.push(round_close((1.0 / l).asin().to_degrees())),
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
            _ => match rpn_evaluate(rpn_from_infix(exp)) {
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

        assert_eq!(rpn_evaluate(rpn_from_infix(r_add)).unwrap(), 9.0, "Add failed");
        assert_eq!(rpn_evaluate(rpn_from_infix(r_sub)).unwrap(), -2.0, "Sub failed");
        assert_eq!(rpn_evaluate(rpn_from_infix(r_mul)).unwrap(), 15.0, "Mul failed");
        assert_eq!(rpn_evaluate(rpn_from_infix(r_div)).unwrap(), 3.0, "Div failed");
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
    fn trigonometry() {
        let r_1 = rpn_from_infix(String::from("sin 90"));
        let r_2 = rpn_from_infix(String::from("cos 180"));
        let r_3 = rpn_from_infix(String::from("tan 45"));
        let r_4 = rpn_from_infix(String::from("cot 45"));
        let r_5 = rpn_from_infix(String::from("sec 60"));
        let r_6 = rpn_from_infix(String::from("csc 30"));
        let r_7 = rpn_from_infix(String::from("arcsin 0.5"));
        let r_8 = rpn_from_infix(String::from("arccos 0.5"));
        let r_9 = rpn_from_infix(String::from("arctan 1"));
        let r_10 = rpn_from_infix(String::from("arccot 1"));
        let r_11 = rpn_from_infix(String::from("arcsec 2"));
        let r_12 = rpn_from_infix(String::from("arccsc 2"));

        assert_eq!(rpn_evaluate(r_1).unwrap(), 1.0, "sin failed");
        assert_eq!(rpn_evaluate(r_2).unwrap(), -1.0, "cos failed");
        assert_eq!(rpn_evaluate(r_3).unwrap(), 1.0, "tan failed");
        assert_eq!(rpn_evaluate(r_4).unwrap(), 1.0, "cot failed");
        assert_eq!(rpn_evaluate(r_5).unwrap(), 2.0, "sec failed");
        assert_eq!(rpn_evaluate(r_6).unwrap(), 2.0, "csc failed");
        assert_eq!(rpn_evaluate(r_7).unwrap(), 30.0, "arcsin failed");
        assert_eq!(rpn_evaluate(r_8).unwrap(), 60.0, "arccos failed");
        assert_eq!(rpn_evaluate(r_9).unwrap(), 45.0, "arctan failed");
        assert_eq!(rpn_evaluate(r_10).unwrap(), 45.0, "arccot failed");
        assert_eq!(rpn_evaluate(r_11).unwrap(), 60.0, "arcsec failed");
        assert_eq!(rpn_evaluate(r_12).unwrap(), 30.0, "arccsc failed");
    }

    #[test]
    fn negative() {
        let r_1 = rpn_from_infix(String::from("-1+1"));
        let r_2 = rpn_from_infix(String::from("10^-2"));
        let r_3 = rpn_from_infix(String::from("15 * (-4)"));

        assert_eq!(rpn_evaluate(r_1).unwrap(), 0.0, "Start of expression");
        assert_eq!(rpn_evaluate(r_2).unwrap(), 0.01, "After binary operator");
        assert_eq!(rpn_evaluate(r_3).unwrap(), -60.0, "After opening parenthese");
    }
}
