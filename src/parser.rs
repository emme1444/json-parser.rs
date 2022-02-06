pub mod node;
mod tokenizer;

use std::collections::HashMap;

use self::node::{
    ArrayNode, BooleanNode, Node, NullNode, NumberNode, ObjectNode, Span, StringNode,
};
use self::tokenizer::{Token, TokenKind, Tokenizer};

pub struct Parser<'source> {
    tokenizer: Tokenizer<'source>,
    // tokens: Option<Vec<Token>>,
    tokens: Vec<Token>,
    index: usize,
    source: Option<&'source String>,
}

impl<'source> Parser<'source> {
    // TODO: support dep. injecting the Tokenizer? Make Tokenizer a trait?
    // pub fn new(tokenizer: Option<Tokenizer>) -> Parser {
    fn new(comments: bool) -> Parser<'source> {
        Parser {
            tokenizer: Tokenizer::new(comments),
            tokens: vec![],
            index: 0,
            source: None,
        }
    }

    pub fn new_without_comments() -> Parser<'source> {
        Parser::new(false)
    }

    pub fn new_with_comments() -> Parser<'source> {
        Parser::new(true)
    }

    fn reset(&mut self) {
        self.tokens = vec![];
        self.index = 0;
        self.source = None;
    }

    // TODO: for the future: how do I communicate that this only returns
    //       either Ok(Node::Array) or Ok(Node::Object)? I guess the only way is to doc it
    pub fn parse(&mut self, source: &'source String) -> Result<Node, String> {
        self.reset();

        self.source = Some(source);

        self.tokens = self.tokenizer.tokenize(self.source.unwrap())?;

        match self.current().kind() {
            TokenKind::OpenSquareBracket => self.parse_array_literal().map(Node::Array),
            TokenKind::OpenCurlyBrace => self.parse_object_literal().map(Node::Object),
            // TODO: obviously there has to be way more info here!
            _ => Err(format!(
                "unexpected token: must be array literal or object literal"
            )),
        }
    }

    fn parse_value(&mut self) -> Result<Node, String> {
        match self.current().kind() {
            TokenKind::NullLiteral => self.parse_null_literal().map(Node::Null),
            TokenKind::BooleanLiteral => self.parse_boolean_literal().map(Node::Boolean),
            TokenKind::NumberLiteral => self.parse_number_literal().map(Node::Number),
            // FIXME: do this for all of these!
            TokenKind::StringLiteral => self.parse_string_literal().map(Node::String),
            TokenKind::OpenSquareBracket => self.parse_array_literal().map(Node::Array),
            TokenKind::OpenCurlyBrace => self.parse_object_literal().map(Node::Object),
            _ => panic!("this should not happens! tf?"),
        }
    }

    fn parse_object_literal(&mut self) -> Result<ObjectNode, String> {
        let start = self.consume(TokenKind::OpenCurlyBrace)?.span().start();
        let mut map = HashMap::new();

        // TODO: See what happens with this if we don't terminate with a square bracket
        while self.current().kind() != &TokenKind::ClosedCurlyBrace
            && self.current().kind() != &TokenKind::Eoi
        {
            // let key = self.consume(TokenKind::StringLiteral)?;
            // the below is easier since it's already removed the quotes
            let key = self.parse_string_literal()?;
            self.consume(TokenKind::Colon)?;
            let value = self.parse_value()?;
            // TODO: maybe handle if this doesn't return None? since we can't store dupes
            map.insert(key.value, value);
            if self.current().kind() != &TokenKind::ClosedCurlyBrace {
                self.consume(TokenKind::Comma)?;
            }
        }

        let end = self.consume(TokenKind::ClosedCurlyBrace)?.span().end();

        Ok(ObjectNode {
            span: Span::new(start, end),
            value: map,
            raw: self.source.unwrap()[start.cursor()..end.cursor()].to_string(),
        })
    }

    fn parse_array_literal(&mut self) -> Result<ArrayNode, String> {
        let start = self.consume(TokenKind::OpenSquareBracket)?.span().start();
        let mut array = Vec::new();

        // TODO: See what happens with this if we don't terminate with a square bracket
        while self.current().kind() != &TokenKind::ClosedSquareBracket
            && self.current().kind() != &TokenKind::Eoi
        {
            let value = self.parse_value()?;
            // FIXME: This only allows trailing commas, which is not compliant
            array.push(value);
            if self.current().kind() != &TokenKind::ClosedSquareBracket {
                self.consume(TokenKind::Comma)?;
            }
        }

        let end = self.consume(TokenKind::ClosedSquareBracket)?.span().end();

        Ok(ArrayNode {
            span: Span::new(start, end),
            value: array,
            raw: self.source.unwrap()[start.cursor()..end.cursor()].to_string(),
        })
    }

    fn parse_string_literal(&mut self) -> Result<StringNode, String> {
        self.consume(TokenKind::StringLiteral).map(|token| {
            let raw = token.raw();
            StringNode {
                raw: raw.to_string(),
                // FIXME: Maybe extract this, and also think about if there are any edge-cases
                value: raw[1..raw.len() - 1].to_string(),
                span: *token.span(),
            }
        })
    }

    fn parse_number_literal(&mut self) -> Result<NumberNode, String> {
        self.consume(TokenKind::NumberLiteral).map(|token| {
            let raw = token.raw();
            NumberNode {
                raw: raw.to_string(),
                value: raw
                    .to_string()
                    .parse()
                    .expect("could not parse number literal raw value"),
                span: *token.span(),
            }
        })
    }

    fn parse_boolean_literal(&mut self) -> Result<BooleanNode, String> {
        self.consume(TokenKind::BooleanLiteral).map(|token| {
            let raw = token.raw();
            BooleanNode {
                raw: raw.to_string(),
                value: raw
                    .to_string()
                    .parse()
                    .expect("could not parse boolean literal raw value"),
                span: *token.span(),
            }
        })
    }

    fn parse_null_literal(&mut self) -> Result<NullNode, String> {
        self.consume(TokenKind::NullLiteral).map(|token| NullNode {
            raw: token.raw().to_string(),
            span: *token.span(),
        })
    }

    fn peek<'a>(&'a self, offset: usize) -> &'a Token {
        // fn peek(&self, offset: usize) -> &'source Token {
        self.tokens
            .get(self.index + offset)
            // .get(offset)
            .expect("probably index out of range when peeking next token")
    }

    fn current<'a>(&'a self) -> &'a Token {
        self.peek(0)
    }

    /// consume and expect a specific token kind, returning the token of said kind,
    /// or an error message if the next token was not of the expected kind.
    // fn consume<'a>(&'a mut self, kind: TokenKind) -> Result<&'a Token, String> {
    fn consume(&mut self, kind: TokenKind) -> Result<&Token, String> {
        // self.index += 1;

        // let token = self.current();
        // FIXME: This would be the above, but I'm having problems with ownership
        let token = self.tokens.get(self.index).expect("hello");

        if token.kind() != &kind {
            // TODO: add more detail
            return Err(format!(
                "unexpected token: found `{:?}`, expected `{:?}`",
                token.kind(),
                &kind
            )
            .to_string());
        }

        self.index += 1;

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::node::Position;
    use super::*;

    #[test]
    fn it_parses_null() {
        let mut parser = Parser::new_without_comments();

        let ast = parser
            .parse(&"null".to_string())
            .expect("could not parse null");

        assert_eq!(
            ast,
            Node::Null(NullNode {
                span: Span::new(Position::start(), Position::new(5, 1, 5)),
                raw: "null".to_string(),
            })
        );
    }
}
