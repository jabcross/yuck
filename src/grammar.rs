use crate::traits::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SimpleGrammar <'source> {
    // pub source: &'source str,
    pub productions: HashMap<&'source str, Vec<Vec<&'source str>>>,
}

impl <'source> SimpleGrammar <'source> {
    pub fn from_source(source: &'source str) -> Self {
        let mut _self = SimpleGrammar {
            // source,
            productions: HashMap::new(),
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
        _self
    }

    pub fn pretty_print(&self, type_info: &TypeInfo)->String {
        String::new()
    }
}

impl<'source> Grammar<'source> for SimpleGrammar<'source> {

    fn get_productions(&self, id: &'source str)-> &Vec<Vec<&'source str>> {
        &self.productions[id]
    }
}