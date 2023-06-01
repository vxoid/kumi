use crate::{types::Type, node::Node, context::Context};
use std::io;

#[derive(Clone)]
pub struct Variable {
    value: Type,
    name: String
}

impl Variable {
    pub fn new(name: &str, value: Node, context: &mut Context) -> io::Result<Self> {
        Ok(Self { value: value.get_value(context)?, name: name.to_string() })
    }

    pub fn get_value(&self) -> &Type {
        &self.value
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}