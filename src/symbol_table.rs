use std::collections::HashMap;
use std::io;

use crate::variable::Variable;

#[derive(Clone)]
pub struct SymbolTable<'p> {
    parent: Option<&'p Self>,
    symbols: HashMap<String, Variable>
}

impl<'p> SymbolTable<'p> {
    pub fn new(parent: &'p Self) -> Self {
        Self { parent: Some(parent), symbols: HashMap::new() }
    }

    pub fn get_from_tree(&self, name: &str) -> io::Result<&Variable> {    
        let result = self.symbols.get(name);

        result.map_or(self.parent.map_or(Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("there isn\'t variable with name {}", name)
        )), |result| result.get_from_tree(name)), |variable| Ok(variable))
    }

    pub fn declare(&mut self, variable: Variable) {
        self.symbols.insert(variable.get_name().to_string(), variable);
    }

    pub fn remove(&mut self, name: &str) -> io::Result<()> {
        self.symbols.remove(name).map_or(Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("there isn\'t variable with name {}", name)
        )), |_| Ok(()))
    }
}

impl<'p> Default for SymbolTable<'p> {
    fn default() -> Self {
        Self { parent: None, symbols: HashMap::new() }
    }
}