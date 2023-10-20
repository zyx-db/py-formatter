use std::fmt::format;

use super::expression::Expression;

type Depth = u16;

pub struct Monostack {
    val: Vec<(Depth, Expression)>, 
}

impl Monostack {
    pub fn new() -> Monostack{
        Monostack{val: Vec::new()}
    }

    pub fn push(&mut self, e: Expression, d: Depth){
        if self.val.len() == 0{
            self.val.push((d, e));
            return;
        }
        let (prev_depth, _) = self.val.last().unwrap();

        if d >= *prev_depth {
            self.val.push((d, e));
        }
        // TODO
        // Here we have to group until we can push
        else {

        }
    }

    pub fn compress(&mut self) -> String{
        format!("{:?}", self.val)
    }
}
