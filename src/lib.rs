type Expression = String;

pub fn group_expressions(expressions: Vec<Expression>) -> Expression {
    let expression_list = expressions
        .iter()
        .map(|e| format!("n({})", e))
        .collect::<Vec<String>>()
        .join(",");

    format!("g([{}])", expression_list)
}
