extern crate proc_macro;
use proc_macro::{TokenStream, Group, Punct, TokenTree};

use syn::{ItemMacro, parse_macro_input};
use quote::{quote};

#[proc_macro_attribute]
pub fn instrument_macro(_args: TokenStream, input: TokenStream ) -> TokenStream {
    let macro_decl = parse_macro_input!(input as syn::ItemMacro);
    eprintln!("Parsed macro def!");
    let unparsed_tokens: TokenStream = proc_macro::TokenStream::from(macro_decl.mac.tokens);
    eprintln!("{:#?}", unparsed_tokens);
    let mut rules: Vec<DeclRule> = Vec::new();
    let parsed_rules = process_rules(unparsed_tokens, &mut rules);

    match parsed_rules {
        Ok(_rules_vec) => {eprintln!("Finished parsing!")},
        Err(msg) => {eprintln!("{:?}", msg)},
    }
    quote!().into()
}

fn process_rules(rule_tokens: TokenStream, output_rules: &mut Vec<DeclRule>) -> Result<&mut Vec<DeclRule>, ParseFailure> {
    let mut iter = rule_tokens.into_iter();
    let consumed_iter : usize = 2;

    let mut parsed = false;
    let mut count = 0;
    eprintln!("Beginning parsing!");
    while ! parsed {
        count += 1;
        let item = iter.next();
        let input = match item {
            Some(TokenTree::Group(item)) => {Some(item) },
            _ => None,
        };

        if Option::is_none(&input) {return Err(ParseFailure::Left);} 

        // count += 1;
        // let item = iter.next();
        // let eq_tok = match item {
        //     Some(TokenTree::Punct(item)) => Some(item),
        //     _ => None,
        // };
        // if Option::is_none(&eq_tok) { return Err("Parse error");}
        // count += 1;
        // let item = iter.next();
        // let ge_tok = match item {
        //     Some(TokenTree::Punct(item)) => Some(item),
        //     _ => None,
        // };
        // if Option::is_none(&ge_tok) {return Err("Parse Error");}
        // count += 1;
        // let item = iter.next();
        // let body = match item {
        //     Some(TokenTree::Group(item)) => Some(item),
        //     _ => None,
        // };
        // if Option::is_none(&body) {return Err("Parse Error");}
        // count += 1;

        // let item = iter.next();
        // let semi_tok = match item {
        //     Some(TokenTree::Punct(item)) => Some(item),
        //     _ => None,
        // };
        // if Option::is_none(&semi_tok) {return Err("Parse error") ; }

        // eprintln!("Count: {}", count);
        // output_rules.push(
        //     DeclRule{
        //         input: input.unwrap(),
        //         eq_tok: eq_tok.unwrap(),
        //         ge_tok: ge_tok.unwrap(),
        //         body: body.unwrap(),
        //         semi_tok: semi_tok.unwrap()
        //     }
        // );

        if count != consumed_iter {
            parsed = false;
        } else {
            parsed = true;
        }
    }
    Ok(output_rules)
}

// we want every 5 items to be popped into the thing, and we know what the format should be, so we need to:
// Pop each item, match the tokentree enum variant. If it's correct, we can simply return it, otherwise None it
struct DeclRule {
    input: Group,
    eq_tok: Punct,
    ge_tok: Punct,
    body: Group,
    semi_tok: Punct,
}

#[derive(Debug)]
enum ParseFailure {
    Left,
    Eq,
    Ge,
    Right,
    SemiColon,
}