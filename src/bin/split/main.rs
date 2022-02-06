use regex::Regex;

use parser::parser::node::Position;

const NEWLINE_SPLIT_PATTERN: &str = "(\n\r|\n)";

fn main() {
    // let s = "something\nsomething else\n\na";
    let s = "";
    println!("{}", s.len());
    let split_regex = Regex::new(NEWLINE_SPLIT_PATTERN).unwrap();
    let substrings: Vec<_> = split_regex.split(s).into_iter().collect();
    println!("{:#?}", substrings);

    let mut pos = Position::start();
    pos.add_from_str(s);
    println!("{:#?}", pos);
}
