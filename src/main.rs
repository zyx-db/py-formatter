use std::{fs::File, io::{BufReader, BufRead, Write}};

mod parsing;
use parsing::parser::parse;

fn main() {
    let input = File::open("in.py").unwrap();
    let buffered = BufReader::new(input);

    let one_line = parse(buffered.lines().map(|x| x.unwrap()).collect());

    let mut output = File::create("out.py").unwrap();
    write!(output, "{}", one_line).unwrap();
}
