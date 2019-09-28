use crate::node::*;
use std::collections::HashSet;

pub type Mark = (usize, usize);

pub trait Grammar<'source> {
    fn get_productions(&self, id: &'source str)-> &Vec<Vec<&'source str>>;
    fn get_file_symbol(&self) -> &'source str;
    fn get_starting_tokens(&self, id: &'source str) -> &HashSet<&'source str>;
    fn get_min_token_count(&self,
    pattern: &[&'source str]) -> usize;
}

pub trait Lexer <'source> {
    fn generate_tokens<'code> (&self, source: &'code str) 
        -> Vec<Node<'source, 'code>>;
    fn get_type_count(&self)->usize;
}
