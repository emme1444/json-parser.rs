use core::fmt::Debug;
use regex::Regex;
use std::collections::HashMap;

// const NEWLINE_SPLIT_PATTERN: Regex = Regex::new("(\n\r|\n)").unwrap();
const NEWLINE_SPLIT_PATTERN: &str = "(\n\r|\n)";

// pub enum Node {
//     Null,
//     Bool,
//     Number,
//     String,
//     Array,
//     Object,
// }

/// Represents a position within the source string
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    cursor: usize,
    line: usize,
    column: usize,
}

impl Position {
    pub fn new(cursor: usize, line: usize, column: usize) -> Position {
        if line == 0 {
            panic!("line must not be zero")
        }

        Position {
            cursor,
            line,
            column,
        }
    }

    /// Initializes values to their starting position for some source.
    pub fn start() -> Position {
        Position {
            cursor: 0,
            line: 1,
            column: 0,
        }
    }

    // TODO: create a new function for position that calculates
    //       `cursor` from `line` and `column` based on source
    // pub fn from_source(source: &String, line: u32, column: u32) -> Position {
    //     Position {
    //         cursor: (),
    //         line: (),
    //         column: (),
    //     }
    // }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    fn add_cursor(&mut self, amount: usize) {
        self.cursor += amount;
    }

    pub fn add_lines(&mut self, amount: usize) {
        self.line += amount;
    }

    pub fn add_line_and_cursor(&mut self) {
        self.add_lines_and_cursor(1);
    }

    // FIXME: This definitely does not do what it should. i think something to do with
    //        the state of the column left after the lines added
    pub fn add_lines_and_cursor(&mut self, amount: usize) {
        self.line += amount;
        self.add_cursor(amount);
        self.reset_column();
    }

    // pub fn add_lines_with_total(&mut self, lines: usize, total: usize) {
    //     if lines < 1 {
    //         panic!("cannot use Position::add_lines_with_total when lines is < 1");
    //     }

    //     self.line += lines;
    //     self.cursor += total;
    //     self.reset_column();
    // }

    pub fn reset_column(&mut self) {
        self.column = 0;
    }

    pub fn add_column(&mut self) {
        self.add_columns(1);
    }

    pub fn add_columns(&mut self, amount: usize) {
        self.column += amount;
        self.add_cursor(amount);
    }

    pub fn add_from_str(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }

        // FIXME: This never changes but can not be stored, compiled, as a constant.
        //        We should probably consider moving this into the Tokenizer instance.
        let split_regex = Regex::new(NEWLINE_SPLIT_PATTERN).unwrap();
        let lines: Vec<_> = split_regex.split(s).into_iter().collect();

        if lines.len() == 1 {
            self.add_columns(lines.first().unwrap().len());
            return;
        }

        // let crlf_matches = s.matches("\n\r");
        // let lf_matches = s.matches("\n");

        // let crlf_count = crlf_matches.count();
        // let lf_count = lf_matches.count();

        // self.add_lines_with_total(crlf_count + lf_count, s.len());
        // self.add_lines_with_total(lines.len() - 1, s.len());

        // let last_line = lines.pop().unwrap();

        // this should always return something since we would have returned
        // the function previously if the len was < 2.
        let (&last_line, _) = lines.split_last().unwrap();

        self.add_lines(lines.len() - 1);
        self.add_cursor(s.len());

        // we're modifying self.column directly, as to bypass
        // self.add_columns's call to self.add_cursor. The reason
        // being that we've already advanced cursor manually
        // with the full string (s) length.
        self.column += last_line.len();

        // TODO: 1. split string by lines
        //       2. advance self by len of each substring plus a new line per substring
        // let split_regex = Regex::new(NEWLINE_SPLIT_PATTERN).unwrap();
        // let substrings: Vec<_> = split_regex.split(s).into_iter().collect();

        // match substrings.len() {
        //     0 => {} // or return,
        //     1 => {
        //         // only one line: increment only column (and cursor)
        //         self.add_columns(substrings.first().unwrap().len());
        //     }
        //     _ => {
        //         // "something\nsomething else\n\n" ->
        //         //   [
        //         //     "something",
        //         //     "something else",
        //         //     "",
        //         //     "",
        //         //   ]
        //         let (substrings, last_substrings) = substrings.split_at(substrings.len() - 1);
        //         let &last_substring = last_substrings
        //             .first()
        //             // .expect("not enough items in last_substrings");
        //             .unwrap();

        //         for &substring in substrings {
        //             self.add_columns(substring.len());
        //             self.add_line();
        //         }

        //         self.add_columns(last_substring.len());
        //         // self.add_line(); // this would bad!
        //     }
        // }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Span {
    start: Position,
    end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Span {
        Span { start, end }
    }

    /// A collapsed span where both positions are initialized
    /// to the start of some source.
    pub fn empty() -> Span {
        Span {
            start: Position::start(),
            end: Position::start(),
        }
    }

    /// A Span collapsed at the given position.
    pub fn collapsed(position: Position) -> Span {
        Span {
            start: position,
            end: position,
        }
    }

    // TODO: We need the amount of lines for this.
    //       And the number of characters for the last line.
    // pub fn full_source(source: &String) -> Span {
    //     Span {
    //         start: Position::beginning(),
    //         end: Position {
    //             position: (),
    //             line: (),
    //             chr: (),
    //         },
    //     }
    // }

    pub fn start(&self) -> Position {
        self.start
    }

    pub fn end(&self) -> Position {
        self.end
    }
}

// TODO: what do we name this?
// pub trait Spanning {
//     fn span(&self) -> &Span;
// }

// pub trait Node: Debug {
//     fn span(&self) -> &Span;
// }

#[derive(Debug, PartialEq)]
pub enum Node {
    Null(NullNode),
    Boolean(BooleanNode),
    Number(NumberNode),
    String(StringNode),
    Array(ArrayNode),
    Object(ObjectNode),
}

impl Node {
    pub fn is_null(&self) -> bool {
        match self {
            Node::Null(_) => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            Node::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Node::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Node::String(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Node::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            Node::Object(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct NullNode {
    pub span: Span,
    pub raw: String, // this will always be "null"
}

#[derive(Debug, PartialEq)]
pub struct BooleanNode {
    pub span: Span,
    pub value: bool,
    pub raw: String, // this will always be either "false" or "true"
}

#[derive(Debug, PartialEq)]
pub struct NumberNode {
    pub span: Span,
    pub value: NumberNodeValue,
    pub raw: String,
}

#[derive(Debug, PartialEq)]
pub enum NumberNodeValue {
    Float(f64),
    Int(i64),
}

#[derive(Debug, PartialEq)]
pub struct StringNode {
    pub span: Span,
    pub value: String,
    pub raw: String, // this includes the quotes ("")
}

#[derive(Debug, PartialEq)]
pub struct ArrayNode {
    pub span: Span,
    pub value: Vec<Node>, // TODO: does this need to be Box<Node>?
    pub raw: String,      // this includes the square brackets ([])
}

#[derive(Debug, PartialEq)]
pub struct ObjectNode {
    pub span: Span,
    // pub value: Vec<ObjectEntry>, // TODO: does this need to be Box<Node>?
    pub value: HashMap<String, Node>, // TODO: this will be String for now
    pub raw: String,                  // this includes the curly braces ({})
}

// // TODO: is this a node?
// #[derive(Debug, PartialEq, Eq)]
// pub struct ObjectEntry {
//     pub key: String, // or is this StringNode? let's go String for now!
//     pub value: Node, // TODO: does this need to be Box<Node>?
// }

// #[derive(Debug, PartialEq, Eq)]
// pub struct NullNode {
//     span: Span,
// }

// impl NullNode {
//     pub fn new(span: Span) -> NullNode {
//         NullNode { span }
//     }
// }

// impl Node for NullNode {
//     fn span(&self) -> &Span {
//         &self.span
//     }
// }

// #[derive(Debug, PartialEq, Eq)]
// pub struct BoolNode {
//     span: Span,
//     value: bool,
//     raw: String,
// }

// impl BoolNode {
//     pub fn new(span: Span, value: bool, raw: String) -> BoolNode {
//         BoolNode { span, value, raw }
//     }
// }

// impl Node for BoolNode {
//     fn span(&self) -> &Span {
//         &self.span
//     }
// }

// #[derive(Debug, PartialEq)]
// pub enum Number {
//     Int(isize),
//     Float(f64),
// }

// #[derive(Debug, PartialEq)]
// pub struct NumberNode {
//     span: Span,
//     value: Number,
//     raw: String,
// }

// impl NumberNode {
//     pub fn new(span: Span, value: Number, raw: String) -> NumberNode {
//         NumberNode { span, value, raw }
//     }
// }

// impl Node for NumberNode {
//     fn span(&self) -> &Span {
//         &self.span
//     }
// }

// // Override equal because of ObjectNode keys
// #[derive(Debug, PartialEq, Eq)]
// pub struct StringNode {
//     span: Span,
//     value: String,
//     raw: String, // this includes the quotation marks ("<value>")
// }

// impl StringNode {
//     pub fn new(span: Span, value: String, raw: String) -> StringNode {
//         StringNode { span, value, raw }
//     }
// }

// impl Node for StringNode {
//     fn span(&self) -> &Span {
//         &self.span
//     }
// }

// #[derive(Debug)]
// pub struct ArrayNode {
//     span: Span,
//     value: Vec<Box<dyn Node>>,
//     raw: String, // this includes the square brackets ([])
// }

// impl ArrayNode {
//     pub fn new(span: Span, value: Vec<Box<dyn Node>>, raw: String) -> ArrayNode {
//         ArrayNode { span, value, raw }
//     }
// }

// impl Node for ArrayNode {
//     fn span(&self) -> &Span {
//         &self.span
//     }
// }

// #[derive(Debug)]
// pub struct ObjectNode {
//     span: Span,
//     value: HashMap<StringNode, Box<dyn Node>>,
//     raw: String, // this includes the curly braces ({})
// }

// impl ObjectNode {
//     pub fn new(span: Span, value: HashMap<StringNode, Box<dyn Node>>, raw: String) -> ObjectNode {
//         ObjectNode { span, value, raw }
//     }
// }

// impl Node for ObjectNode {
//     fn span(&self) -> &Span {
//         &self.span
//     }
// }
