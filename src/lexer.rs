use crate::{token::*, keyword::Keyword, types::Type, op::Op};

use std::io;

const DIGITS: &'static str = "0123456789";
const NUM_SYMBOLS: &'static str = ".";

pub struct Lexer {
    text: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    cc: Option<char>
}

impl Lexer {
    pub fn new(text: &str) -> Self {
        let text: Vec<char> = text.chars().collect();
        let pos = 0;
        let cc = text.get(pos).map(|&c| c);
        Self {
            text,
            line: 0,
            col: 0,
            pos,
            cc
        }
    }

    pub fn step(&mut self) {
        self.pos += 1;
        self.col += 1;

        if self.cc == Some('\n') {
            self.line += 1;
            self.col = 0
        }

        self.cc = self.text.get(self.pos).map(|&c| c);
    }

    fn read_number(&mut self) -> io::Result<TT> {
        let mut str = String::new();
        let mut dots = 0;

        while let Some(cc) = self.cc {
            if !DIGITS.contains(cc) && !NUM_SYMBOLS.contains(cc) {
                break
            }

            if cc == '.' {
                if dots == 1 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput, format!("can\'t float can have only 1 dot can\'t add one more to {}", str)
                    ));
                }
                dots += 1;
            }

            str.push(cc);
            self.step()
        }

        if dots == 0 {
            return Ok(TT::Type(Type::Int(str.parse().map_err(
                |err| io::Error::new(io::ErrorKind::InvalidInput, format!("can\'t parse {} as i128 due to {}", str, err))
            )?)))
        }
        
        Ok(TT::Type(Type::Float(str.parse().map_err(
            |err| io::Error::new(io::ErrorKind::InvalidInput, format!("can\'t parse {} as f64 due to {}", str, err))
        )?)))
    }

    fn read_indenifier(&mut self) -> TT {
        let mut indenifier = String::new();

        while let Some(cc) = self.cc {
            if !cc.is_alphabetic() && !cc.is_ascii_digit() && cc != '_' {
                break
            }

            indenifier.push(cc);
            
            self.step()
        }

        let map = Keyword::hash_map();
        if let Some(keyword) = map.get(&indenifier) {
            return TT::Keyword(keyword.clone())
        }

        return TT::Indenifier(indenifier);
    }

    fn read_lt(&mut self) -> TT {
        self.step();

        if let Some('=') = self.cc {
            self.step();
            return TT::Op(Op::LTE);
        }

        TT::Op(Op::LT)
    }

    fn read_gt(&mut self) -> TT {
        self.step();

        if let Some('=') = self.cc {
            self.step();
            return TT::Op(Op::GTE);
        }

        TT::Op(Op::GT)
    }

    fn read_not(&mut self) -> TT {
        self.step();

        if let Some('=') = self.cc {
            self.step();
            return TT::Op(Op::NE);
        }

        TT::Op(Op::Not)
    }

    fn read_or(&mut self) -> io::Result<TT> {
        self.step();

        if let Some('|') = self.cc {
            self.step();
            return Ok(TT::Op(Op::Or));
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "expected \'|\'"
        ))
    }

    fn read_and(&mut self) -> io::Result<TT> {
        self.step();

        if let Some('&') = self.cc {
            self.step();
            return Ok(TT::Op(Op::And));
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "expected \'&\'"
        ))
    }

    fn read_eq(&mut self) -> TT {
        self.step();

        if let Some('=') = self.cc {
            self.step();
            return TT::Op(Op::Eq);
        }

        return TT::EQ;
    }

    pub fn tokenize(&mut self) -> io::Result<Vec<Token>> {
        let result = self._tokenize();
        
        result.map_err(|err| {
            let line = get_line_by_char_index(&self.text, self.pos).unwrap_or(&[]);
            let pointers = line.into_iter().map(|_| '^').collect::<String>();

            io::Error::new(
                err.kind(),
                format!("...\n{}\n{}\nLexer error on line - {}, column - {}: {}\n...",
                line.into_iter().collect::<String>(), pointers, self.line+1, self.col+1, err.to_string())
            )
        })
    }

    fn _tokenize(&mut self) -> io::Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while let Some(cc) = self.cc {
            if " \t".contains(cc) {
                self.step();
                continue
            } else if DIGITS.contains(cc) {
                let start = self.pos.clone();
                let number = self.read_number()?;
                tokens.push(Token::new(start, self.pos, number));
                continue
            } else if cc.is_alphabetic() || cc == '_' {
                let start = self.pos.clone();
                let indenifier = self.read_indenifier();
                tokens.push(Token::new(start, self.pos, indenifier));
                continue
            }

            match cc {
                '+' => tokens.push(Token::new(self.pos, self.pos+1, TT::Op(Op::Add))),
                '-' => tokens.push(Token::new(self.pos, self.pos+1, TT::Op(Op::Sub))),
                '/' => tokens.push(Token::new(self.pos, self.pos+1, TT::Op(Op::Devd))),
                '*' => tokens.push(Token::new(self.pos, self.pos+1, TT::Op(Op::Mult))),
                '^' => tokens.push(Token::new(self.pos, self.pos+1, TT::Op(Op::Pow))),
                '%' => tokens.push(Token::new(self.pos, self.pos+1, TT::Op(Op::Remain))),
                '(' => tokens.push(Token::new(self.pos, self.pos+1, TT::LPR)),
                ')' => tokens.push(Token::new(self.pos, self.pos+1, TT::RPR)),
                '=' => {
                    let start = self.pos.clone();
                    let eq = self.read_eq();
                    tokens.push(Token::new(start, self.pos, eq));
                    continue;
                },
                '|' => {
                    let start = self.pos.clone();
                    let or = self.read_or()?;
                    tokens.push(Token::new(start, self.pos, or));
                    continue
                },
                '&' => {
                    let start = self.pos.clone();
                    let and = self.read_and()?;
                    tokens.push(Token::new(start, self.pos, and));
                    continue
                },
                '<' => {
                    let start = self.pos.clone();
                    let lt = self.read_lt();
                    tokens.push(Token::new(start, self.pos, lt));
                    continue
                },
                '>' => {
                    let start = self.pos.clone();
                    let gt = self.read_gt();
                    tokens.push(Token::new(start, self.pos, gt));
                    continue
                },
                '!' => {
                    let start = self.pos.clone();
                    let not = self.read_not();
                    tokens.push(Token::new(start, self.pos, not));
                    continue
                }
                _ => return Err(io::Error::new(
                    io::ErrorKind::InvalidInput, format!("invalid character \'{}\'", cc)
                ))
            }
            
            self.step()
        }

        tokens.push(Token::new(self.pos, self.pos, TT::EOF));

        Ok(tokens)
    }
}

pub fn get_line_by_char_index<'c>(s: &'c [char], index: usize) -> Option<&'c [char]> {
    let start = s[..index].into_iter().enumerate().rfind(|(_ , c)| c == &&'\n').map_or(0, |(i, _)| i + 1);
    let end = s[start..].into_iter().enumerate().find(|(_, c)| c == &&'\n').map_or(s.len(), |(i, _)| start + i);
    if end > start {
        Some(&s[start..end])
    } else {
        None
    }
}