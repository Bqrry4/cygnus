use pest::{Parser, error::Error, iterators::Pairs};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct CgParser;

pub fn parse(input: &str) -> Result<Pairs<'_, Rule>, Error<Rule>> {
    CgParser::parse(Rule::program, input)
}
