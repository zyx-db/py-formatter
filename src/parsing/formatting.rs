pub fn one_line_function(lines: Vec<String>) -> String {
    let (function_def, expressions) = lines.split_first().unwrap();
    format!(
        "{}\n\treturn {}",
        fix_function_def(function_def.clone()),
        group_expressions(Vec::from(expressions))
    )
}

fn fix_function_def(function_def: String) -> String {
    let g_def = "g=lambda x: next(filter(lambda x: x is not None, x), None)";
    let n_def = "n=lambda x: None";

    let closing_idx = function_def.rfind(')').unwrap();
    format!(
        "{},{},{}{}",
        &function_def[..closing_idx],
        g_def,
        n_def,
        &function_def[closing_idx..]
    )
}

fn group_expressions(expressions: Vec<String>) -> String {
    if expressions.len() == 1 {
        return clean_expression(expressions[0].clone());
    }
    let expression_list = expressions
        .iter()
        .map(|e| clean_expression(e.clone()))
        .collect::<Vec<String>>()
        .join(",");

    format!("g([{}])", expression_list)
}

fn clean_expression(e: String) -> String {
    // Trim is temporary for now, when we only have one level of expressions
    format!("n({})", fix_assignment(e.trim().to_owned()))
}

fn fix_assignment(e: String) -> String {
    if let Some((mut before_eq, mut after_eq)) = e.split_once('=') {
        if before_eq.chars().last().unwrap() == ':' || after_eq.chars().next().unwrap() == '='  {
            return e
        }
        
        before_eq = before_eq.trim();
        after_eq = after_eq.trim();

        if let (Some(bracket_start), Some(bracket_end)) = (before_eq.rfind('['), before_eq.rfind(']')) {
            format!(
                "{}.__setitem__({},{})",
                &e[..bracket_start],
                &e[bracket_start + 1..bracket_end],
                after_eq
            )
        } else {
            format!("{}:={}", before_eq, after_eq)
        }
    } else {
        e
    }
}
