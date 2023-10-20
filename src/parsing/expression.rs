#[derive(Debug)]
pub enum Expression {
    Function(String, String),
    Statement(String),
    For(String),
    While(String),
    If(String),
    Elif(String),
    Else
}
