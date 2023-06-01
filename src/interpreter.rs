use std::io;

use crate::{
    lexer::Lexer,
    parser::Parser,
    types::Type,
    token::Token,
    context::Context
};

pub struct Interpreter<'c> {
    text: String,
    context: Context<'c>,
    tokens: Vec<Token>
}
impl<'c> Interpreter<'c> {
    pub fn new(text: &str) -> io::Result<Self> {
        let mut lexer = Lexer::new(text);
        let context = Context::new("<program>", None, 1);

        Ok(Self { text: text.to_string(), tokens: lexer.tokenize()?, context })
    }
    
    pub fn update(&mut self, text: &str) -> io::Result<()> {
        self.text = text.to_string();

        let mut lexer = Lexer::new(text);
        self.tokens = lexer.tokenize()?;

        Ok(())
    }

    pub fn run(&mut self) -> io::Result<Type> {
        let mut parser = Parser::new(&self.tokens);

        let node = parser.parse().map_err(|err| err.format(&self.text)).map_err(|err| io::Error::new(
            err.kind(),
            format!("Traceback:\n{}{}", self.context.generate_traceback(), err)
        ))?;

        node.get_value(&mut self.context)
    }
}