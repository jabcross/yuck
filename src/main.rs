mod traits;
mod node;
mod grammar;
mod lexer;
mod parser;
mod misc;

use crate::grammar::*;
use crate::lexer::*;
use crate::parser::*;
use crate::misc::*;
use traits::*;

use std::fs::File;
use std::io::prelude::*;

fn main() {


    let mut grammar_source_file = File::open("./grammar_source.txt").expect("File not found");
    let mut grammar_source = String::new();
    grammar_source_file.read_to_string(&mut grammar_source).expect("Failed to extract file to string");
    println!("{}", grammar_source);

    let mut lexer_source_file = File::open("./lexer_source.txt").expect("File not found");
    let mut lexer_source = String::new();
    lexer_source_file.read_to_string(&mut lexer_source).expect("Failed to extract file to string");
    println!("{}", lexer_source);

    let mut code_source_file = File::open("./code_source.txt").expect("File not found");
    let mut code_source = String::new();
    code_source_file.read_to_string(&mut code_source).expect("Failed to extract file to string");
    println!("{}", code_source);

    let lexer = PatternLexer::from_source(lexer_source.as_str());

    let grammar = SimpleGrammar::from_source(grammar_source.as_str());

    let tokens = lexer
        .generate_tokens(code_source.as_str());

    let tree = parse(&tokens, &grammar, true);

    if let Some(tree) = tree {
        println!("{}",tree.as_string(None));
    }
    else {
        println!("failed to match.");
    }
}
