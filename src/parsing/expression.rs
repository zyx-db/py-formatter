#[derive(Debug)]
pub enum Expression {
    Function(String, String),
    Statement(String),
    For(String, String),
    While(String, String),
    If(String, String),
    Elif(String, String),
    Else(String)
}
