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

// TODO actually return the correct value
// function takes in a line of code, trimmed of whitespace
// determines the type of expression (which enum variant)
// constructs the appropriate variant and returns
fn parse_code(line: &String) -> Option<Expression>{
    match line {
        // function
        s if s.starts_with("def") => {return Some(Expression::Statement(line.clone()));}
        // for
        s if s.starts_with("for") => {return Some(Expression::Statement(line.clone()));}
        // if
        s if s.starts_with("if") => {return Some(Expression::Statement(line.clone()));}
        // elif
        s if s.starts_with("elif") => {return Some(Expression::Statement(line.clone()));}
        // else
        s if s.starts_with("else") => {return Some(Expression::Statement(line.clone()));}
        // while
        s if s.starts_with("while") => {return Some(Expression::Statement(line.clone()));}

        // comments
        s if s.starts_with("#") => { return None; }
        s if s.starts_with("//") => { return None; }

        // any statement
        _ => {return Some(Expression::Statement(line.clone()));}
    }
}
