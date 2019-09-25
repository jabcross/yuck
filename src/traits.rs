use crate::node::*;

pub type Mark = (usize, usize);

pub trait Grammar<'source> {
    fn get_productions(&self, id: &'source str)-> &Vec<Vec<&'source str>>;
}

pub trait Lexer <'source> {
    fn generate_tokens<'code> (&self, source: &'code str)-> Vec<Node<'source, 'code>>;
    fn get_type_count(&self)->usize;
}

pub struct TypeInfo<'source> {
    pub lexer: &'source dyn Lexer<'source>,
    pub grammar: &'source dyn Grammar<'source>
}
