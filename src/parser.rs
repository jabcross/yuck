use crate::node::*;
use crate::grammar::*;
use crate::traits::*;

use std::collections::HashSet;

macro_rules! Node {
    {} => {Node<'source, 'code>};
}

macro_rules! cprint {
    {$context:expr, $($arg:tt)*} => {$context.print_indent(format!($($arg)*).as_str())}; 
}

struct Context{
    indent: usize,
    debug_print: bool
}

impl Context {
    fn print_indent(&self, string: &str) {
        if !self.debug_print {
            return;
        }
        let lines = string.split_terminator("\n").collect::<Vec<_>>();
        for _ in 0..self.indent {
            print!(".   ");
        }
        print!("{}", lines[0]);
        for line in &lines[1..] {
            print!("\n");
            for _ in 0..self.indent {
                print!(".   ");
            }
            print!("{}", line);
        }
        if string.chars().last().unwrap() == '\n' {
            print!("\n");
        }
    }
}

pub fn parse<'source, 'code> (
        tokens: &'code [Node!()],
        grammar: &'source dyn Grammar<'source>,
        debug_print: bool) 
        -> Option<Node!()> {
    println!("Parsing token list:");
    println!("{}",Node::print_node_list(tokens));

    let mut context = Context{indent: 0, debug_print};

    let mut nodes = get_all_matches_symbol(tokens, "file", grammar, &mut context);

    println!("{:?}", nodes.iter().map(|x| x.pretty_print()).collect::<Vec<_>>());

    nodes = nodes.into_iter().filter(|x| x.token_count == tokens.len()).collect();

    if nodes.len() == 0 {
        None
    }
    else {
        nodes.pop()
    }
}

fn get_all_matches_symbol<'source, 'code> (
        tokens: &'code [Node!()],
        symbol: &'source str,
        grammar: &'source (dyn Grammar<'source> + 'source), 
        context: &mut Context)
        -> Vec<Node!()> {
    context.indent += 1;
    let mut rv = vec![];

    cprint!(context, "{} -> {}\n", symbol, Node::print_node_list(tokens));
    
    context.indent += 1;

    if SimpleGrammar::is_token(symbol){
        cprint!(context, "Token match!\n");
        if tokens[0].id == symbol {
            let new_node = tokens[0].clone();
            rv.push(new_node);
        }
    }
    else {
        for production in grammar.get_productions(symbol){
            let matches = get_all_matches_pattern(
                tokens,
                production.as_slice(),
                grammar, context);
            for _match in matches {
                let count = _match.iter().fold(0, |x, y| x + y.token_count);
                let new_node = Node {
                    id: symbol,
                    children: Some(_match),
                    source_slice: None,
                    token_count: count,
                    position: None,
                    data: TokenData::None
                };
                rv.push(new_node);
            }
        }
    }

    context.indent -= 1;

    cprint!(context, "{} -> {} resulted in\n", symbol, Node::print_node_list(tokens));
    cprint!(context, "[\n");
    context.indent += 1;
    for node in &rv {
        use crate::misc::*;
        cprint!(context, "{}\n", node.as_string(None));
    }
    context.indent -= 1;

    cprint!(context, "]\n");

    context.indent -= 1;
    rv
}

fn get_all_matches_pattern<'source, 'code> (
        tokens: &'code [Node!()],
        pattern: &'source [&'source str],
        grammar: &'source (dyn Grammar<'source> + 'source), 
        context: &mut Context)
        -> Vec<Vec<Node!()>> {
    context.indent += 1;

    cprint!(context, "{:?} -> {}\n", pattern, 
    Node::print_node_list(tokens));
    context.indent += 1;

    let first_symbol = pattern[0];
    let pattern_tail = &pattern[1..];

    let first_token = &tokens[0];

    let mut starting_tokens_data = HashSet::new();

    let starting_tokens = if SimpleGrammar::is_token(first_symbol){
        starting_tokens_data.insert(first_symbol);
        &starting_tokens_data
    }
    else {
        grammar.get_starting_tokens(first_symbol)
    };

    let mut rv = vec![];

    if starting_tokens.contains(first_token.id){
        let first_symbol_matches = get_all_matches_symbol(
            tokens,
            first_symbol,
            grammar, context);

        for head_match in first_symbol_matches {
            cprint!(context, "Head matched {}\n", head_match.print_tokens());

            let token_tail = &tokens[head_match.token_count..];
            cprint!(context, "Token tail: {}\n", Node::print_node_list(token_tail));
            cprint!(context, "Pattern tail: {:?}\n", pattern_tail);



            if pattern_tail.len() == 0 {
                rv.push(vec![head_match]);
                continue;
            }

            if grammar.get_min_token_count(pattern_tail) > token_tail.len() {
                cprint!(context, "Too big to fit.\n");
                continue;
            }

            let tail_matches = get_all_matches_pattern(
                token_tail,
                pattern_tail,
                grammar, context);

            for tail_match in tail_matches {
                let mut _match = vec![head_match.clone()];
                _match.extend(tail_match);
                rv.push(_match);
            }

        }
    }

    context.indent -= 1;

    cprint!(context, "{:?} -> {} resulted in\n", pattern, Node::print_node_list(tokens));
    cprint!(context, "[\n");
    context.indent += 1;
    for node_list in &rv {
        cprint!(context, "[\n");
        for node in node_list {
            use crate::misc::*;
            cprint!(context, "{}\n", node.as_string(None));
        }
        cprint!(context, "[\n");
    }
    context.indent -= 1;

    cprint!(context, "]\n");

    context.indent -= 1;
    rv
}
