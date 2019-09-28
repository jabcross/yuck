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

fn main() {

    let grammar_source = "
    file:
    st_l

    st_l:
    st st_l
    st

    st:
    LB
    assign LB

    assign:
    exp ASSIGN_OP exp

    exp:
    INT
    ";

    let lexer_source = "
        INT	[0-9]+
        ASSIGN_OP	=
    ";

    let code = "1 = 1
    ";

    let lexer = PatternLexer::from_source(lexer_source);

    let grammar = SimpleGrammar::from_source(grammar_source);

    let tokens = lexer
        .generate_tokens(code);

    let tree = parse(&tokens, &grammar, false);

    if let Some(tree) = tree {
        println!("{}",tree.as_string(None));
    }
    else {
        println!("failed to match.");
    }
}
