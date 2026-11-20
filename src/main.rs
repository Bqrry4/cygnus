mod parser;

use argh::FromArgs;
use std::{error::Error, fs};

#[derive(FromArgs)]
#[argh(description = r" /\_/\
( O.O )
 > - <")]
struct Args {
    #[argh(positional)]
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = argh::from_env();
    let input = fs::read_to_string(args.path)?;

    let pairs = parser::parse(&input);

    println!("{:#?}", pairs);

    Ok(())
}
