use crate::traits::*;
use std::collections::{HashMap,HashSet};

#[derive(Debug)]
pub struct SimpleGrammar <'source> {
    // pub source: &'source str,
    pub productions: HashMap<&'source str, Vec<Vec<&'source str>>>,
    pub starting_tokens: HashMap<&'source str, HashSet<&'source str>>,
    pub min_token_count: HashMap<&'source str, usize>,
    root_symbol: &'source str
}

impl <'source> SimpleGrammar <'source> {
    pub fn from_source(source: &'source str) -> Self {
        let mut _self = SimpleGrammar {
            // source,
            productions: HashMap::new(),
            starting_tokens: HashMap::new(),
            min_token_count: HashMap::new(),
            root_symbol: ""
        };

        let mut lines = source.split_terminator("\n");

        let mut current_id = "";

        loop {
            let line = lines.next().unwrap().trim();
            if line == "" {
                continue;
            }
            if line.chars().last() != Some(':') {
                panic!("First line of grammar file must start a type");
            }
            else {
                let name = &line[0..line.len()-1];
                current_id = name;
                _self.root_symbol = name;
                _self.productions.insert(name, vec![]);
                break;
            }
        }

        for line in lines {
            if line.trim() == "" {
                continue;
            }
            if let Some(':') = line.chars().last() {
                let line = line.trim();
                let key = &line[..line.len()-1];
                if let None = _self.productions.get(key) {
                    _self.productions.insert(key, vec![]);
                }
                current_id = key
            }
            else {
                let pattern = line.trim().split_whitespace().collect();
                _self.productions.get_mut(current_id).unwrap().push(pattern)
            }

        }

        _self.resolve_limit_tokens();

        println!("Starting tokens:\n{:#?}", _self.starting_tokens);
        println!("Min token count:\n{:#?}", _self.min_token_count);

        _self
    }

    fn resolve_limit_tokens (&mut self) {
        let keys = self.productions.keys().map(|x| *x).collect::<Vec<_>>();
        let mut started: HashSet<&'source str> = HashSet::new();
        let mut ended: HashSet<&'source str> = HashSet::new();
        for key in &keys {
            SimpleGrammar::resolve_symbol(
                key,
                &mut started,
                &mut ended,
                &mut self.starting_tokens,
                &self.productions);
        }

        started.clear();
        ended.clear();

        for key in &keys {
            SimpleGrammar::count_min_tokens(
                key,
                &mut started,
                &mut ended,
                &mut self.min_token_count,
                &self.productions);
        }
    }

    fn resolve_symbol(
            symbol: &'source str,
            started: &mut HashSet<&'source str>,
            ended: &mut HashSet<&'source str>,
            starting_tokens: &mut HashMap<&'source str, HashSet<&'source str>>,
            productions: &HashMap<&'source str, Vec<Vec<&'source str>>>) {
        println!("Resolving symbol {}", symbol);
        if SimpleGrammar::is_token(symbol){
            return;
        }
        if started.contains(symbol){
            return;
        }
        started.insert(symbol);
        if !ended.contains(symbol){
            let mut start_tokens = HashSet::new();
            for production in &productions[symbol] {

                let first_symbol = production[0];
                if SimpleGrammar::is_token(first_symbol){
                    start_tokens.insert(first_symbol);
                }
                else {
                    SimpleGrammar::resolve_symbol(
                        first_symbol,
                        started,
                        ended,
                        starting_tokens,
                        productions);
                    start_tokens.extend(&starting_tokens[first_symbol]);
                }
            }
            starting_tokens.insert(symbol, start_tokens);
            ended.insert(symbol);
        }

    }

    fn count_min_tokens(
        symbol: &'source str,
        started: &mut HashSet<&'source str>,
        ended: &mut HashSet<&'source str>,
        min_token_count: &mut HashMap<&'source str, usize>,
        productions: &HashMap<&'source str, Vec<Vec<&'source str>>>) {
        if SimpleGrammar::is_token(symbol){
            return;
        }
        if started.contains(symbol){
            return;
        }
        started.insert(symbol);
        if !ended.contains(symbol){
            let mut min_token : usize = 99999999;
            for production in &productions[symbol] {
                let mut _min_token = 0;
                let mut recursive = false;
                for &_symbol in production {
                    if SimpleGrammar::is_token(_symbol){
                        _min_token += 1;
                    }
                    else {
                        if ended.contains(_symbol){
                            _min_token += min_token_count[_symbol];
                        }
                        else if started.contains(_symbol){
                            recursive = true;
                            break;
                        }
                        else {
                            SimpleGrammar::count_min_tokens(
                                _symbol,
                                started,
                                ended,
                                min_token_count,
                                productions);
                            _min_token += min_token_count[_symbol];
                        }
                    }
                }
                if recursive {
                    continue;
                }
                min_token = std::cmp::min(min_token, _min_token);
            }
            min_token_count.insert(symbol, min_token);
            ended.insert(symbol);
        }

    }


    pub fn is_token(id: &str) -> bool {
        let c = id.chars().nth(0).unwrap();
        !c.is_lowercase()
    }

    pub fn pretty_print(&self) -> String {
        format!("{:?}", self.productions)
    }
}

impl<'source> Grammar<'source> for SimpleGrammar<'source> {

    fn get_productions(&self, id: &'source str)-> &Vec<Vec<&'source str>> {
        &self.productions[id]
    }

    fn get_file_symbol(&self) -> &'source str {
        &self.root_symbol
    }

    fn get_starting_tokens(&self, id: &'source str) -> &HashSet<&'source str> {
        &self.starting_tokens.get(id).unwrap()
    }

    fn get_min_token_count(&self, pattern: &[&'source str]) -> usize {
        let mut acc: usize = 0;
        for symbol in pattern {
            if SimpleGrammar::is_token(symbol){
                acc += 1;
            }
            else {
                acc += self.min_token_count[symbol];
            }
        }
        acc
    }
}