use crate::{types::Type, variable::Variable, context::Context, op::Op};
use std::io;

#[derive(Clone)]
pub enum Node {
    Number(Type),
    GetVar(String),
    NotOp(Box<Node>),
    UnaryOp(Box<Node>),
    DeclareVar(String, Box<Node>),
    BinOp(Box<Node>, Op, Box<Node>),
}

impl Node {
    pub fn get_value(&self, context: &mut Context) -> io::Result<Type> {
        match self {
            Node::Number(type_) => Ok(type_.clone()),
            Node::UnaryOp(node) => (&*node).get_value(context)?.inverted(),
            Node::DeclareVar(name, value) => {
                let variable = Variable::new(name, (&**value).clone(), context)?;

                context.get_symbol_table_mut().declare(variable);

                Ok(Type::None)
            },
            Node::BinOp(left, op_token, right) => op_token.execute(&(&*left).get_value(context)?, &(&*right).get_value(context)?),
            Node::GetVar(name) => {
                let variable = context.get_symbol_table().get_from_tree(name)?.clone();
                Ok(variable.get_value().clone())
            },
            Node::NotOp(node) => {
                let value = (&*node).get_value(context)?;
                value.ensure_bool(&Type::Bool(true), |a, _| Type::Bool(!a))
                    .map_or(Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("can\'t execute not on not a bool type {}", value.to_string())
                    )), |val| Ok(val))
            }
        }
    }
}