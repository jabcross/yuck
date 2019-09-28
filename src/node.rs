use crate::traits::*;
use crate::lexer::*;
use crate::misc::*;

#[derive(Debug, Clone)]
pub struct Node<'source, 'code> {
    pub id: &'source str,
    pub children: Option<Vec<Node<'source, 'code>>>,
    pub source_slice: Option<&'code str>,
    pub token_count: usize,
    pub position: Option<(Mark, Mark)>,
    pub data: TokenData
}

#[derive(Debug, Clone)]
pub enum TokenData {
    Text(String),
    None
}

impl TokenData {
    pub fn pretty_print(&self)-> String{
        match self {
            TokenData::None => String::new(),
            TokenData::Text(x) => format!("({})",x),
        }
    }
}

impl <'source, 'code> Node<'source, 'code> {
    pub fn from_token(id: &'source str, data: TokenData, slice: &'code str) 
            -> Self {  
        Node {
            id,
            children: None,
            source_slice: Some(slice),
            token_count: 1,
            position: None,
            data
        }
    }

    pub fn pretty_print(&self)->String{
        format!("{}{}",
            self.id,
            self.data.pretty_print())
    }

    pub fn print_node_list(list: &[Node]) -> String {
        let mut rv = String::new();
        if list.len() == 0 {
            rv += "[]";
        }
        else {
            rv.push_str(list.first().unwrap().pretty_print().as_str());
            for node in &list[1..] {
                rv.push_str(" ");
                rv.push_str(node.pretty_print().as_str());
            }
        }
        rv
    }

    pub fn print_tokens(&self) -> String {
        let mut rv = String::new();
        if let Some(children) = &self.children {
            for child in children {
                rv += child.print_tokens().as_str();
                rv += " ";
            }
        }
        else {
            rv += self.id;
        }
        rv
    }

    pub fn print_as_tree(&self, indent: usize) -> String {
        let mut rv  = String::new();
        for _ in 0..indent {
            rv += " ";
        }
        rv += self.pretty_print().as_str();
        rv += "\n";
        if let Some(children) = &self.children {
            for child in children {
                rv += child.print_as_tree(indent + 1).as_str();
            }
        }
        rv
    }
}

impl<'source, 'code> NeatTreePrint for Node <'source, 'code> {
    fn get_children(&self) -> Vec<&dyn NeatTreePrint> {        
        let mut rv : Vec<&dyn NeatTreePrint> = vec![];
        if let Some(children) = &self.children {
            for child in children {
                rv.push(child);
            }
        }
        rv
    }

    fn get_label(&self) -> String {
        self.pretty_print()
    }
}