pub fn group_expressions(expressions: Vec<String>) -> String {
    if expressions.len() == 1 {
        return expressions[0].clone();
    }
    let expression_list = expressions.join(",");

    format!("__([{}])", expression_list)
}

pub fn clean_expression(e: String) -> String {
    if e.starts_with("return") {
        e[6..].trim().to_owned()
    } else {
        format!("_({})", fix_assignment(e.to_owned()))
    }
}

pub fn fix_assignment(e: String) -> String {
    if let Some((mut before_eq, mut after_eq)) = e.split_once('=') {
        if before_eq.chars().last().unwrap() == ':' || after_eq.chars().next().unwrap() == '=' {
            return e;
        }

        before_eq = before_eq.trim();
        after_eq = after_eq.trim();

        if let (Some(bracket_start), Some(bracket_end)) =
            (before_eq.rfind('['), before_eq.rfind(']'))
        {
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

pub fn is_block_start(line: &str) -> bool {
    ["if", "elif", "else", "for", "while", "def"]
        .into_iter()
        .any(|keyword| line.trim().starts_with(keyword))
}

pub fn leading_spaces(s: &String) -> usize {
    s.chars()
        .take_while(|ch| ch.is_whitespace() && *ch != '\n')
        .count()
}

pub fn fix_function_def(function_def: String) -> String {
    let g_def = "__=lambda x:next(filter(lambda x:x is not None,x),None)";
    let n_def = "_=lambda x:None";

    let closing_idx = function_def.rfind(')').unwrap();

    if function_def.chars().nth(closing_idx - 1).unwrap() == '(' {
        format!(
            "{}{},{}{}",
            &function_def[..closing_idx],
            g_def,
            n_def,
            &function_def[closing_idx..]
        )
    } else {
        format!(
            "{},{},{}{}",
            &function_def[..closing_idx],
            g_def,
            n_def,
            &function_def[closing_idx..]
        )
    }
}
