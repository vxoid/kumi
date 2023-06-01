use crate::keyword::Keyword;
use crate::lexer::get_line_by_char_index;
use crate::op::Op;
use crate::token::{TT, Token};
use crate::types::Type;
use crate::node::Node;
use std::io;

pub struct ParserError {
    token: Token,
    err: io::Error
}

impl ParserError {
    pub fn new(token: Token, err: io::Error) -> Self {
        Self { token, err }
    }

    pub fn format(&self, text: &str) -> io::Error {
        let chars: Vec<char> = text.chars().collect();
        let start = self.token.get_start().clone();
        let end = self.token.get_end().clone();

        let line = get_line_by_char_index(&chars, start).unwrap_or(&[]);
        let pointers = line.into_iter().map(|_| '^').collect::<String>();

        io::Error::new(
            self.err.kind(),
            format!(
                "...\n{}\n{}\nParser error on \"{}\", token - {:?}: {}\n...",
                line.into_iter().collect::<String>(),
                pointers,
                &text[start..end],
                self.token.get_tt(),
                self.err
            )
        )
    }
}

pub struct Parser<'t> {
    tokens: &'t [Token],
    pos: usize,
    ct: Option<&'t Token>
}

impl<'t> Parser<'t> {
    pub fn new(tokens: &'t [Token]) -> Self {
        let pos = 0;
        let ct = tokens.get(pos);
        Self { tokens, pos, ct }
    }

    pub fn step(&mut self) {
        self.pos += 1;
        self.ct = self.tokens.get(self.pos)
    }

    pub fn atom(&mut self) -> Result<Node, ParserError> {
        let ct = match self.ct {
            Some(ct) => ct,
            None => return Err(ParserError::new((&self.tokens[self.tokens.len()-1]).clone(), io::Error::new(
                io::ErrorKind::InvalidInput,
                "can\'t parse one more token, reached end"
            ))),
        };
        let tt = ct.get_tt();

        if let TT::Type(Type::Int(_) | Type::Float(_)) = tt {
            self.step();
            let number = ct.get_tt().clone().try_into().map_err(|err| ParserError::new(
                ct.clone(),
                err
            ))?;
            return Ok(Node::Number(number));
        } else if let TT::Indenifier(indentifier) = tt {
            self.step();
            return Ok(Node::GetVar(indentifier.clone()));
        } else if let TT::LPR = tt {
            self.step();
            let expr = self.expr()?;
            if let Some(TT::RPR) = self.ct.map(|value| value.get_tt()) { 
                self.step();
                return Ok(expr);
            } else {                
                return Err(ParserError::new(self.ct.unwrap_or(&self.tokens[self.pos-1]).clone(), io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "expected \')\'"
                )));
            }
        }

        Err(ParserError::new(ct.clone(), io::Error::new(
            io::ErrorKind::InvalidInput,    
            "expected int or float"
        )))
    }

    pub fn power(&mut self) -> Result<Node, ParserError> {
        self.bin_op(
            |token| match token {
                TT::Op(Op::Pow) => true,
                _ => false,
            },
            |parser| parser.atom(),
            |parser| parser.factor(),
        )
    }

    pub fn factor(&mut self) -> Result<Node, ParserError> {
        let ct = match self.ct {
            Some(ct) => ct,
            None => return Err(ParserError::new((&self.tokens[self.tokens.len()-1]).clone(), io::Error::new(
                io::ErrorKind::InvalidInput,
                "can\'t parse one more token, reached end"
            ))),
        };

        if let TT::Op(Op::Sub) = ct.get_tt() {
            self.step();
            let factor = self.factor()?;
            return Ok(Node::UnaryOp(Box::new(factor)));
        }

        self.power()
    }

    pub fn term(&mut self) -> Result<Node, ParserError> {
        self.bin_op_same(|token| match token {
            TT::Op(Op::Devd | Op::Mult | Op::Remain) => true,
            _ => false,
        }, |parser| parser.factor())
    }

    pub fn logic_expr(&mut self) -> Result<Node, ParserError> {
        if let Some(TT::Op(Op::Not)) = self.ct.map(|ct| ct.get_tt()) {
            self.step();

            let node = self.logic_expr()?;

            return Ok(Node::NotOp(Box::new(node)));
        }

        self.bin_op_same(|token| match token {
            TT::Op(type_) => type_.logical(),
            _ => false,
        }, |parser| parser.arithm_expr())
    }

    pub fn arithm_expr(&mut self) -> Result<Node, ParserError> {
        self.bin_op_same(|token| match token {
            TT::Op(Op::Add | Op::Sub) => true,
            _ => false,
        }, |parser| parser.term())
    }

    pub fn expr(&mut self) -> Result<Node, ParserError> {
        if let Some(TT::Keyword(Keyword::Let)) = self.ct.map(|ct| ct.get_tt()) {
            self.step();
            let (name, value) = self.parse_variable()?;

            return Ok(Node::DeclareVar(name, Box::new(value)));
        }

        self.bin_op_same(|token| match token {
            TT::Op(Op::And | Op::Or) => true,
            _ => false,
        }, |parser| parser.logic_expr())
    }

    pub fn parse(&mut self) -> Result<Node, ParserError> {
        let mut result = self.expr();

        if result.is_ok() {
            match self.ct.map(|ct| ct.get_tt()) {
                Some(TT::EOF) => {},
                _ => result = Err(ParserError::new(self.ct.unwrap_or(&self.tokens[self.tokens.len()-1]).clone(), io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "expected \'+\', \'-\', \'/\', \'*\', \'^\' or \'%\'"
                )))
            }
        }

        result
    }

    fn parse_variable(&mut self) -> Result<(String, Node), ParserError> {
        let indentifier = match self.ct.map(|ct| ct.get_tt()) {
            Some(TT::Indenifier(indenifier)) => indenifier,
            _ => return Err(ParserError::new(self.ct.unwrap_or(&self.tokens[self.pos-1]).clone(), io::Error::new(
                io::ErrorKind::InvalidInput,
                "expected indentifier"
            ))),
        };

        self.step();

        match self.ct.map(|ct| ct.get_tt()) {
            Some(TT::EQ) => {},
            _ => return Err(ParserError::new(self.ct.unwrap_or(&self.tokens[self.pos-1]).clone(), io::Error::new(
                io::ErrorKind::InvalidInput,
                "expected EQ"
            ))),
        }

        self.step();
        let expr = self.expr()?;
        
        Ok((indentifier.clone(), expr))
    }

    fn bin_op<W, FA, FB>(&mut self, mut wl: W, mut func_a: FA, mut func_b: FB) -> Result<Node, ParserError>
        where W: FnMut(&TT) -> bool, FA: FnMut(&mut Self) -> Result<Node, ParserError>, FB: FnMut(&mut Self) -> Result<Node, ParserError> {        
        let mut left = func_a(self)?;
        
        while let Some(ct) = self.ct {
            if !wl(ct.get_tt()) {
                break
            }

            self.step();
            let right = func_b(self)?;
            left = Node::BinOp(Box::new(left), ct.get_tt().clone().try_into().map_err(|err| ParserError::new(
                ct.clone(),
                err
            ))?, Box::new(right))
        }

        Ok(left)
    }

    fn bin_op_same<W, F>(&mut self, mut wl: W, mut func: F) -> Result<Node, ParserError>
        where W: FnMut(&TT) -> bool, F: FnMut(&mut Self) -> Result<Node, ParserError> + Clone {
        let mut left = func(self)?;
        
        while let Some(ct) = self.ct {
            if !wl(ct.get_tt()) {
                break
            }

            self.step();
            let right = func(self)?;
            left = Node::BinOp(Box::new(left), ct.get_tt().clone().try_into().map_err(|err| ParserError::new(
                ct.clone(),
                err
            ))?, Box::new(right))
        }

        Ok(left)
    }
}