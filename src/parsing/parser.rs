use super::{monostack::Monostack, expression::Expression, formatting::one_line_function};

pub fn parse(lines: Vec<String>) -> String {
    let mut stack = Monostack::new();
    for line in lines{
        if let Some((expression, depth)) = evaluate(line){
            stack.push(expression, depth);
        }
    }
    return stack.compress();
}

fn evaluate(line: String) -> Option<(Expression, u16)> {
    let depth = leading_spaces(&line);
    if let Some(expr) = parse_code(&line.trim().to_owned()){
        return Some((expr, depth));
    }
    None 
}

fn leading_spaces(s: &String) -> u16{
    let val = s
        .chars()
        .take_while(|ch| ch.is_whitespace() && *ch != '\n')
        .count();
    val as u16
}

// TODO find the locations of the notable portions of expressions
// function takes in a line of code, trimmed of whitespace
// determines the type of expression (which enum variant)
// constructs the appropriate variant and returns
fn parse_code(line: &String) -> Option<Expression>{
    // blank line
    if line.is_empty(){ return None; } 

    // Note the space at the end of the compared values.
    // This ensures that this function avoids false positives
    match line {
        s if s.starts_with("def ")=> {
            let func_name = "";
            let func_args = "";
            return Some(Expression::Function(func_name.to_owned(), func_args.to_owned()));
        }
        s if s.starts_with("for ") => {
            let loop_condition = "";
            return Some(Expression::For(loop_condition.to_owned())); 
        }
        s if s.starts_with("if ") => {
            let condition = "";
            return Some(Expression::If(condition.to_owned()));
        }
        s if s.starts_with("elif ") => {
            let condition = "";
            return Some(Expression::Elif(condition.to_owned()));
        }
        // in this case the line is simply:
        // else:
        // note that there is no space
        s if s.starts_with("else:") => {
            return Some(Expression::Else);
        }
        s if s.starts_with("while ") => {
            let condition = "";
            return Some(Expression::While(condition.to_owned()));
        }

        // comments
        s if s.starts_with("#") => { return None; }
        s if s.starts_with("//") => { return None; }

        // any statement
        _ => {return Some(Expression::Statement(line.clone()));}
    }
}
