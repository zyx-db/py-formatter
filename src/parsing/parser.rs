use super::{monostack::Monostack, expression::Expression, formatting::one_line_function};

pub fn parse(lines: Vec<String>) -> String {
    return one_line_function(lines);
    // let mut stack = Monostack::new();
    // for line in lines{
    //     if let Some((expression, depth)) = evaluate(line){
    //         stack.push(expression, depth);
    //     }
    // }
    // return stack.compress();
}

fn evaluate(line: String) -> Option<(Expression, u16)> {
    if let Some(expr) = parse_code(&line){
        let depth = leading_spaces(&line);
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

// TODO 
// function takes in a line of code, trimmed of whitespace
// determines the type of expression (which enum variant)
// constructs the appropriate variant and returns
fn parse_code(line: &String) -> Option<Expression>{
    None
}
