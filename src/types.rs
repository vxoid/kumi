use std::{fmt::Debug, io};

#[derive(Clone)]
pub enum Type {
    Int(i128),
    Float(f64),
    Bool(bool),
    None
}

impl Type {
    pub fn inverted(&self) -> io::Result<Self> {
        match self {
            Type::Int(i) => Ok(Type::Int(-i)),
            Type::Float(f) => Ok(Type::Float(-f)),
            Type::Bool(b) => Ok(Type::Bool(!b)),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("can\'t invert {}", self.to_string())
            ))
        }
    }

    pub fn ensure_float<F>(&self, token: &Self, func: F) -> Option<Self> where F: FnOnce(&f64, &f64) -> Self {
        match self {
            Type::Float(a) => match token {
                Type::Float(b) => Some(func(a, b)),
                _ => None
            },
            _ => None
        }
    }

    pub fn ensure_int<F>(&self, token: &Self, func: F) -> Option<Self> where F: FnOnce(&i128, &i128) -> Self {
        match self {
            Type::Int(a) => match token {
                Type::Int(b) => Some(func(a, b)),
                _ => None
            },
            _ => None
        }
    }

    pub fn ensure_bool<F>(&self, token: &Self, func: F) -> Option<Type> where F: FnOnce(&bool, &bool) -> Self {
        match self {
            Type::Bool(a) => match token {
                Type::Bool(b) => Some(func(a, b)),
                _ => None
            },
            _ => None
        }
    }

    pub fn add(&self, other: &Self) -> io::Result<Self> {
        let number = match (self.ensure_float(other, |a, b| Type::Float(a+b)), self.ensure_int(other, |a, b| Type::Int(a+b))) {
            (None, Some(i)) => i,
            (Some(f), None) => f,
            _ => return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("can\'t execute add operation on {} and {}, the types must match", self.to_string(), other.to_string()
            )))
        };
        
        Ok(number)
    }

    pub fn sub(&self, other: &Self) -> io::Result<Self> {
        let number = match (self.ensure_float(other, |a, b| Type::Float(a-b)), self.ensure_int(other, |a, b| Type::Int(a-b))) {
            (None, Some(i)) => i,
            (Some(f), None) => f,
            _ => return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("can\'t execute subtract operation on {} and {}, the types must match", self.to_string(), other.to_string()
            )))
        };
        
        Ok(number)
    }

    pub fn devd(&self, other: &Self) -> io::Result<Self> {
        if other.is_zero() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("can\'t devide {:?} by zero ({:?})", self, other)
            ));
        }
        let result = (self.ensure_float(other, |a, b| Type::Float(a/b)), self.ensure_int(other, |a, b| Type::Int(a/b)));
    
        let number = match result {
            (None, Some(i)) => i,
            (Some(f), None) => f,
            _ => return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("can\'t execute devide operation on {} and {}, the types must match", self.to_string(), other.to_string()
            )))
        };
        
        Ok(number)
    }

    pub fn mult(&self, other: &Self) -> io::Result<Self> {
        let number = match (self.ensure_float(other, |a, b| Type::Float(a*b)), self.ensure_int(other, |a, b| Type::Int(a*b))) {
            (None, Some(i)) => i,
            (Some(f), None) => f,
            _ => return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("can\'t execute multiply operation on {} and {}, the types must match", self.to_string(), other.to_string()
            )))
        };
        
        Ok(number)
    }

    pub fn pow(&self, other: &Self) -> io::Result<Self> {
        let number = match (self.ensure_float(other, |a, b| Type::Float(a.powf(*b))), self.ensure_int(other, |a, b| Type::Int(a.pow(*b as u32)))) {
            (None, Some(i)) => i,
            (Some(f), None) => f,
            _ => return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("can\'t execute power operation on {} and {}, the types must match", self.to_string(), other.to_string()
            )))
        };
        
        Ok(number)
    }

    pub fn remain(&self, other: &Self) -> io::Result<Self> {
        let number = match (self.ensure_float(other, |a, b| Type::Float(a%b)), self.ensure_int(other, |a, b| Type::Int(a%b))) {
            (None, Some(i)) => i,
            (Some(f), None) => f,
            _ => return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("can\'t execute remainder operation on {} and {}, the types must match", self.to_string(), other.to_string()
            )))
        };
        
        Ok(number)
    }

    pub fn eq(&self, other: &Self) -> io::Result<Self> {
        let number = match (self.ensure_float(other, |a, b| Type::Bool(a==b)), self.ensure_int(other, |a, b| Type::Bool(a==b))) {
            (None, Some(i)) => i,
            (Some(f), None) => f,
            _ => return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("can\'t execute equals operation on {} and {}, the types must match", self.to_string(), other.to_string()
            )))
        };
        
        Ok(number)
    }

    pub fn and(&self, other: &Self) -> io::Result<Self> {
        other.ensure_bool(other, |a, b| Type::Bool(*a && *b)).map_or(Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("can\'t execute and operation on {} and {}, the types must match", self.to_string(), other.to_string()
        ))), |out| Ok(out))
    }

    pub fn or(&self, other: &Self) -> io::Result<Self> {
        other.ensure_bool(other, |a, b| Type::Bool(*a || *b)).map_or(Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("can\'t execute or operation on {} and {}, the types must match", self.to_string(), other.to_string()
        ))), |out| Ok(out))
    }


    pub fn is_zero(&self) -> bool {
        match self {
            Type::Int(i) => i == &0,
            Type::Float(f) => f == &0.0,
            _ => false
        }
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::None => "()".to_string(),
            Type::Int(i) => format!("int({})", i),
            Type::Bool(b) => format!("bool({})", b),
            Type::Float(f) => format!("float({})", f),
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}