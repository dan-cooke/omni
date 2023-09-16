use logos::{Logos, SpannedIter};

use crate::{errors::LexicalError, tokens::Token};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    token_stream: SpannedIter<'input, Token>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(token, span)| {
            let start = span.start;
            let end = span.end;
            let token = match token {
                Ok(token) => token,
                Err(err) => return Err(err),
            };

            Ok((start, token, end))
        })
    }
}
