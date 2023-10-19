pub enum Expression {
    Function(String),
    Statement(String),
    For(String, String),
    If(String, String),
    Elif(String, String),
    Else(String)
}
