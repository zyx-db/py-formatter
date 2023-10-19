use super::expression::Expression;

type Depth = u8;

pub struct Monostack {
    val: Vec<(Depth, Expression)>, 
}

impl Monostack {
    pub fn new() -> Monostack{
        Monostack{val: Vec::new()}
    }

    pub fn push(&mut self, e: Expression, d: Depth){}

    pub fn compress(&mut self) -> String{
        "".to_owned()
    }
}
