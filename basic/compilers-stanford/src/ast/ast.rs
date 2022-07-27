use std::fmt::{Display, Formatter};
use crate::define::define::{NType, TokenType};

pub trait E: Display {}
pub trait S: Display {}

pub(crate) struct VarStatements {
    pub(crate) m_type: NType,
    pub(crate) identifier: Option<Box<dyn E>>,
    pub(crate) init: Option<Box<dyn E>>,
}

impl Display for VarStatements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "m_type:{:?},identifier:{},init:{{{}}}",
            self.m_type,
            self.identifier.as_ref().unwrap(),
            self.init.as_ref().unwrap()
        )
    }
}

impl S for VarStatements {}

#[derive(Debug)]
pub(crate) struct ID {
    pub(crate) name: String,
    pub(crate) xtype: NType,
}

impl Display for ID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{xtype:{:?},name:{},struct:ID}}", self.xtype, self.name)
    }
}

impl E for ID {}

#[derive(Debug)]
pub struct Literal {
    pub(crate) m_type: NType,
    pub(crate) value: String,
}

#[derive(Debug)]
pub struct Strings {
    pub(crate) m_type: NType,
    pub(crate) value: String,
}

impl Strings {
    pub(crate) fn new() -> Strings {
        Strings {
            m_type: NType::Strings,
            value: "".to_string(),
        }
    }
}

impl Literal {
    pub(crate) fn new() -> Literal {
        Literal {
            m_type: NType::Int,
            value: "".to_string(),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{m_type:{:?},value:{},struct:Literal}}",
            self.m_type, self.value
        )
    }
}

impl E for Literal {}

pub(crate) struct OperatorExpress {
    pub(crate) left: Option<Box<dyn E>>,
    pub(crate) operator: String,
    pub(crate) right: Option<Box<dyn E>>,
}
impl E for OperatorExpress {}

impl Display for OperatorExpress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OperatorExpress{{left:{},Operator:{},right:{} }}",
            self.left.as_ref().unwrap(),
            self.operator,
            self.right.as_ref().unwrap()
        )
    }
}


pub struct ArrayExpress {
    pub(crate) m_type: TokenType,
    pub(crate) body:Vec<Box<dyn E>>
}

impl Display for ArrayExpress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl E for ArrayExpress{

}


pub struct BlockStatement {
    pub(crate) m_type: TokenType,
    pub(crate) body:Vec<Box<dyn E>>
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl S for BlockStatement{

}

pub struct IfStatement{
    pub(crate) test:Option<Box<dyn E>>,
    pub(crate) alternate:Option<Box<dyn S>>,
}

impl Display for IfStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl S for IfStatement{

}