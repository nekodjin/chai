use std::io::Read;

use chumsky::{Parser, Stream};

mod model;
mod parsing;

fn main() {
    use logos::Logos as _;

    use parsing::token::Token;

    let mut input = String::new();

    std::io::stdin().read_to_string(&mut input).unwrap();

    let input = input;
    let tokens = Token::lexer(&input).spanned().collect::<Vec<_>>();

    // println!("{tokens:#?}");

    let eoi_span = input.len()..input.len();
    let stream = Stream::from_iter(eoi_span, tokens.into_iter());
    let parser = parsing::parser::parser();

    let module = parser.parse(stream).unwrap();

    println!("{module:#?}")
}
