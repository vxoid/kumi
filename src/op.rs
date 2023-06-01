use crate::types::Type;
use std::io;

#[derive(Clone)]
pub enum Op {
    Add,
    Sub,
    Devd,
    Mult,
    Pow,
    Remain,
    Not,
    NE,
    Eq,
    LT,
    LTE,
    GT,
    GTE,
    And,
    Or
}

impl Op {
    pub fn execute(&self, a: &Type, b: &Type) -> io::Result<Type> {        
        match self {
            Op::Add => a.add(b),
            Op::Sub => a.sub(b),
            Op::Devd => a.devd(b),
            Op::Mult => a.mult(b),
            Op::Pow => a.pow(b),
            Op::Remain => a.remain(b),
            Op::And => a.and(b),
            Op::Or => a.or(b),
            Op::Eq => a.eq(b),
            Op::NE => a.eq(b).map(|b| b.inverted())?,
            _ => return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("can\'t execute {} on {} and {}", self.to_string(), a.to_string(), b.to_string())
            ))
        }
    }

    pub fn logical(&self) -> bool {
        match self {
            Op::Eq | Op::NE | Op::LT | Op::LTE | Op::GT | Op::GTE => true,
            _ => false
        }
    }
}

impl ToString for Op {
    fn to_string(&self) -> String {
        match self {
            Op::Add => "+".to_string(),
            Op::Sub => "-".to_string(),
            Op::Devd => "/".to_string(),
            Op::Mult => "*".to_string(),
            Op::Pow => "^".to_string(),            
            Op::Remain => "%".to_string(),
            Op::Eq => "==".to_string(),
            Op::Not => "!".to_string(),
            Op::NE => "!=".to_string(),
            Op::LT => "<".to_string(),
            Op::LTE => "<=".to_string(),
            Op::GT => ">".to_string(),
            Op::GTE => ">=".to_string(),
            Op::And => "&&".to_string(),
            Op::Or => "||".to_string(),
        }
    }
}