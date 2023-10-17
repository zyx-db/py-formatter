use py_formatter::group_expressions;
fn main() {
    let expressions = vec!["x = 5".to_owned(), "y = 5".to_owned()];
    println!("{}", group_expressions(expressions));
}
