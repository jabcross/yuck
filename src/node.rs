use crate::traits::*;
use crate::lexer::*;

#[derive(Debug)]
pub struct Node<'source, 'code> {
    pub id: &'source str,
    pub children: Option<Vec<Node<'source, 'code>>>,
    pub slice: Option<&'code str>,
    pub position: Option<(Mark, Mark)>,
    pub data: TokenData
}

#[derive(Debug)]
pub enum TokenData {
    I32(i32),
    None
}

impl TokenData {
    pub fn pretty_print(&self)-> String{
        match self {
            TokenData::None => String::new(),
            TokenData::I32(x) => format!("({})",x),
        }
    }
}

impl <'source, 'code> Node<'source, 'code> {
    pub fn from_token(id: &'source str, data: TokenData, slice: &'code str) 
            -> Self {  
        Node {
            id,
            children: None,
            slice: Some(slice),
            position: None,
            data
        }
    }

    pub fn pretty_print(&self, type_info: &TypeInfo)->String{
        format!("{}{}",
            self.id,
            self.data.pretty_print())
    }
}