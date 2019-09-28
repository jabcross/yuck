use crate::traits::*;
use crate::node::*;

use regex::{Regex, Match};
use std::collections::HashMap;

#[derive(Debug)]
pub struct PatternLexer <'source> {
    token_patterns: HashMap<&'source str, TokenPattern>,
}

#[derive(Debug)]
pub struct TokenPattern {
    name: String,
    pattern: Option<Regex>
}

impl <'source> PatternLexer <'source> {
    pub fn from_source(source: &'source str) -> Self {
        let mut _self = PatternLexer{
            token_patterns: HashMap::new(),
        };

        // Add built-in tokens

        _self.token_patterns.insert("LB",
            TokenPattern {
                name: String::from("LB"),
                pattern: None
            }
        );

        // Parse source 

        let pairs = source.split_terminator("\n")
            .filter(|x| *x!="")
            .map(|x| x
                .trim()
                .split_terminator("\t")
                .collect::<Vec<_>>())
            .filter(|x| x.len() == 2);

        for pair in pairs {
            println!("{:?}", pair);
            let name = String::from(pair[0]);
            let regex = format!("^{}",pair[1]);
            _self.token_patterns.insert(pair[0], 
                TokenPattern {
                    name,
                    pattern: Some(Regex::new(regex.as_str()).unwrap())
                }
            );
        }

        _self
    }
}

impl<'source> Lexer<'source> for PatternLexer<'source> {

    fn generate_tokens<'code> (&self, code: &'code str)-> Vec<Node<'source, 'code>>{
        let mut slice = code;
        let whitespace_pattern = Regex::new("^[ \t]+").unwrap();
        let mut row = 0;
        let mut column = 0;
        let mut offset = 0;
        let mut tokens = vec![];
        loop {
            if slice.len() == 0 {
                break;
            }

            // remove starting whitespace
            if let Some(_match) = whitespace_pattern.find_at(slice, 0) {
                println!("Found some whitespace.");
                let whitespace_length = _match.as_str().len();
                offset += whitespace_length;
                column += whitespace_length;
                slice = &slice[whitespace_length..];
                println!("Remaining slice: {}", slice);
                continue;
            }

            if &slice[0..1] == "\n" {
                tokens.push(Node::from_token(
                    "LB",
                    TokenData::None,
                    &slice[0..1]
                ));
                row += 1;
                column = 0;
                offset += 1;
                slice = &slice[1..];
                continue;
            }

            let mut found = false;

            for (&id, token_pattern) in &self.token_patterns {
                if let Some(regex) = &token_pattern.pattern {
                    println!("Trying to match {} to {}", slice, regex);
                    if let Some(_match) = regex.find_at(slice, 0){
                        let token_slice = &slice[0.._match.as_str().len()];
                        let capture = regex.captures(_match.as_str());
                        let token_data = if let Some(_capture) = capture {
                            if _capture.len() > 1 {
                                let s = &_capture[1];
                                TokenData::Text(String::from(s))
                            }
                            else {
                                TokenData::None
                            }
                        }
                        else {
                            TokenData::None
                        };

                        tokens.push(Node::from_token(
                            id,
                            token_data,
                            token_slice));
                        slice = &slice[token_slice.len()..];
                        offset += token_slice.len();
                        column += token_slice.len();
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                panic!("Couldn't match");
            }

        }
        tokens
    }

    fn get_type_count(&self)->usize{
        self.token_patterns.len()
    }

}