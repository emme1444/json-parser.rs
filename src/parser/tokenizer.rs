use regex::Regex;

use super::node::{Position, Span};

pub struct TokenSpec {
    kind: TokenKind,
    regex: Regex,
}

impl TokenSpec {
    fn new(kind: TokenKind, regex: &'static str) -> TokenSpec {
        TokenSpec {
            kind,
            regex: Regex::new(regex).unwrap(),
        }
    }

    fn test(&self, s: &String) -> Option<String> {
        self.regex.find(s).map(|m| m.as_str().to_string())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    /// End of Input
    Eoi,
    NewLine,
    WhiteSpace,
    LineComment,
    BlockComment,
    Comma,
    Colon,
    OpenSquareBracket,
    ClosedSquareBracket,
    OpenCurlyBrace,
    ClosedCurlyBrace,
    NullLiteral,
    BooleanLiteral,
    NumberLiteral,
    StringLiteral,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    raw: String,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, raw: String, span: Span) -> Token {
        Token { kind, raw, span }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn raw(&self) -> &String {
        &self.raw
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

pub struct Tokenizer<'source> {
    specs: Vec<TokenSpec>,
    position: Position,
    source: Option<&'source String>,
    comments: bool,
}

impl<'source> Tokenizer<'source> {
    pub fn new(comments: bool) -> Tokenizer<'source> {
        Tokenizer {
            specs: vec![
                TokenSpec::new(TokenKind::NewLine, r"^\n\r"),
                TokenSpec::new(TokenKind::NewLine, r"^\n"),
                TokenSpec::new(TokenKind::WhiteSpace, r"^[\t\f\v ]+"),
                TokenSpec::new(TokenKind::LineComment, r"^//[^\n\r]*"),
                TokenSpec::new(TokenKind::BlockComment, r"^/\*[\s\S]*?\*/"),
                TokenSpec::new(TokenKind::Comma, "^,"),
                TokenSpec::new(TokenKind::Colon, "^:"),
                TokenSpec::new(TokenKind::OpenSquareBracket, r"^\["),
                TokenSpec::new(TokenKind::ClosedSquareBracket, r"^\]"),
                TokenSpec::new(TokenKind::OpenCurlyBrace, r"^\{"),
                TokenSpec::new(TokenKind::ClosedCurlyBrace, r"^\}"),
                // look-around not supported
                // TokenSpec::new(TokenKind::NullLiteral, r"^\bnull(?!\w|\$)"),
                TokenSpec::new(TokenKind::NullLiteral, r"^\bnull\b"),
                // look-around not supported
                // TokenSpec::new(TokenKind::BooleanLiteral, r"^\bfalse(?!\w|\$)"),
                TokenSpec::new(TokenKind::BooleanLiteral, r"^\bfalse\b"),
                // look-around not supported
                // TokenSpec::new(TokenKind::BooleanLiteral, r"^\btrue(?!\w|\$)"),
                TokenSpec::new(TokenKind::BooleanLiteral, r"^\btrue\b"),
                TokenSpec::new(TokenKind::NumberLiteral, r"^\d+\.\d+"),
                TokenSpec::new(TokenKind::NumberLiteral, r"^\d+"),
                // TODO: Support escaping quotes
                TokenSpec::new(TokenKind::StringLiteral, "^\"[^\"]*\""),
                TokenSpec::new(TokenKind::StringLiteral, "^'[^']*'"),
            ],
            position: Position::start(),
            source: None,
            comments,
        }
    }

    /// Reset the tokenizers's source related fields to their defaults,
    /// preparing for a new tokenize run.
    fn reset(&mut self) {
        self.position = Position::start();
        self.source = None;
    }

    // TODO: we could make this lazy? but i'm tired
    pub fn tokenize(&mut self, source: &'source String) -> Result<Vec<Token>, String> {
        self.reset();

        self.source = Some(source);

        let mut result = Vec::new();
        while !self.has_reached_end_of_source() {
            let token = self.get_token()?;

            // TODO: for now i'm ignoring whitespace stuff here
            //       I thought maybe I could do it in the parser. but eh
            //       Like I can't think of a use for these in the parser this is why
            if let TokenKind::NewLine | TokenKind::WhiteSpace = token.kind() {
                continue;
            } else if let TokenKind::LineComment | TokenKind::BlockComment = token.kind() {
                if !self.comments {
                    // TODO: Perhaps this should some kind of "unrecognized token" stuff?
                    return Err(format!("comments are not supported"));
                }

                continue;
            }

            result.push(token);
        }

        // TODO: actually will this ever get reached; I guess if something really weird happened
        //       i'm not even sure this is the correct logic, but for a placeholder; whatever
        // verify that we have consumed the full source string; if not return error
        // if self.position.cursor() < self.source.unwrap().len() {
        //     return Err("did not consume full source".to_string());
        // }

        // at last push end of input token
        result.push(Token::new(
            TokenKind::Eoi,
            // "<EOI>".to_string(), // or empty string: String::new():
            String::new(),
            Span::collapsed(self.position),
        ));

        println!("{:#?}", result);

        Ok(result)
    }

    fn get_token(&mut self) -> Result<Token, String> {
        let s = &self.source.unwrap()[self.position.cursor()..].to_string();

        // old position - where the current token starts
        let old_position = self.position;

        // TODO: This can maybe be done using iterators?
        for spec in &self.specs {
            // TODO: ...or, at least, this maybe?
            match spec.test(&s) {
                Some(m) => {
                    // advance the position/cursor
                    match spec.kind {
                        TokenKind::NewLine => self.position.add_line_and_cursor(),
                        TokenKind::BlockComment => self.position.add_from_str(&m),
                        _ => self.position.add_columns(m.len()),
                    }

                    return Ok(Token::new(
                        spec.kind,
                        m,
                        Span::new(old_position, self.position),
                    ));
                }
                None => continue,
            }
        }

        // FIXME: This error message should be so much better formatted.
        //        Like; probably get the whole line we're on, if the line is
        //        much longer than necessary omit leading and trailing based on
        //        some threshold. Then, kind of like how the Rust compiler does
        //        it, basically show the line (maybe truncated right) and a line
        //        below it showing a pointer to the current (self.) position of
        //        the character.
        Err(format!("unrecognized token: rest of input was '{}'", s))
    }

    fn has_reached_end_of_source(&self) -> bool {
        self.position.cursor() >= self.source.unwrap().len()
    }
}
