use std::{fs::File, io::{BufReader, BufRead, Write}};

use py_formatter::one_line_function;
fn main() {
    let input = File::open("in.py").unwrap();
    let buffered = BufReader::new(input);

    let one_line = one_line_function(buffered.lines().map(|x| x.unwrap()).collect());

    let mut output = File::create("out.py").unwrap();
    write!(output, "{}", one_line).unwrap();
}
