mod traits;
mod node;
mod grammar;
mod lexer;

use crate::lexer::*;
use crate::grammar::*;
use traits::*;

fn main() {

    let grammar_source = "
    file:
    statement

    statement:
    assignment

    assignment:
    expression ASSIGN expression

    expression:
    LITERAL
    ";

    let lexer_source = "
        INTEGER	[0-9]+
        ASSIGN	=
    ";

    let code = "
        1 = 1
    ";


    let lexer = PatternLexer::from_source(lexer_source);

    let grammar = SimpleGrammar::from_source(grammar_source);

    let type_info = TypeInfo{
        lexer: &lexer,
        grammar: &grammar
    };

    let tokens = lexer
        .generate_tokens(code)
        .into_iter()
        .filter(|x| x.id != "LINEBREAK")
        .collect::<Vec<_>>();

    println!("{:?}",
        tokens
        .iter()
        .map(|x| x.pretty_print(&type_info))
        .collect::<Vec<_>>());

}
