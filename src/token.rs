use std::fmt::Debug;
use std::io;

use crate::keyword::Keyword;
use crate::op::Op;
use crate::types::Type;

#[derive(Clone)]
pub struct Token {
    tt: TT,
    start: usize,
    end: usize
}

impl Token {
    pub fn new(start: usize, end: usize, tt: TT) -> Self {
        Self { tt, start, end }
    }

    pub fn get_start(&self) -> &usize {
        &self.start
    }

    pub fn get_end(&self) -> &usize {
        &self.end
    }

    pub fn get_tt(&self) -> &TT {
        &self.tt
    }
}

#[derive(Clone)]
pub enum TT {
    RPR,
    LPR,
    Op(Op),
    Type(Type),
    Keyword(Keyword),
    Indenifier(String),
    EQ,
    EOF
}

impl TryInto<Type> for TT {
    type Error = io::Error;
    
    fn try_into(self) -> Result<Type, Self::Error> {
        match self {
            TT::Type(type_) => Ok(type_),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("can\'t convert {} to type", self.to_string())
            ))
        }
    }
}

impl TryInto<Op> for TT {
    type Error = io::Error;
    
    fn try_into(self) -> Result<Op, Self::Error> {
        match self {
            TT::Op(op) => Ok(op),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("can\'t convert {} to operator", self.to_string())
            ))
        }
    }
}

impl ToString for TT {
    fn to_string(&self) -> String {
        match self {
            TT::RPR => ")".to_string(),
            TT::LPR => "(".to_string(),
            TT::Op(op) => op.to_string(),
            TT::Type(type_) => type_.to_string(),
            TT::Keyword(keyword) => keyword.to_string(),
            TT::Indenifier(indenifier) =>  indenifier.clone(),
            TT::EQ => "=".to_string(),
            TT::EOF => "EOF".to_string(),
        }
    }
}

impl Debug for TT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}