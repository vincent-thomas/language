use logos::{Logos, Span, SpannedIter};
use std::{iter::Peekable, num::ParseIntError};

use crate::parser::Ident;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
  InvalidInteger(String),
  #[default]
  NonAsciiCharacter,
}
/// Error type returned by calling `lex.slice().parse()` to u8.
impl From<ParseIntError> for LexingError {
  fn from(err: ParseIntError) -> Self {
    use std::num::IntErrorKind::*;
    match err.kind() {
      PosOverflow | NegOverflow => {
        LexingError::InvalidInteger("overflow error".to_owned())
      }
      _ => LexingError::InvalidInteger("other error".to_owned()),
    }
  }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
#[logos(error = LexingError)]
pub enum Tokens {
  // Item Prefixes
  #[token("fn")]
  Fn,

  #[token("class")]
  Class,

  #[token("enum")]
  Enum,

  // Scopes
  #[token("{")]
  BlockOpen,

  #[token("}")]
  BlockClose,

  #[token("(")]
  ParamOpen,

  #[token(")")]
  ParamClose,

  // Breakers
  #[token(",")]
  Comma,

  #[token(";")]
  Semicolon,

  // Operators
  #[token("=")]
  AssignmentOperator,

  // Ident
  #[regex("[a-zA-Z]+", |item| Ident::new(item.slice()))]
  Ident(Ident),

  // Operators
  #[token("let")]
  Let,
}

pub struct Lexer<'a>(Peekable<SpannedIter<'a, Tokens>>);

impl<'a> Lexer<'a> {
  pub fn from_source(src: &'static str) -> Self {
    let inner_lexer = Tokens::lexer(src);
    Self(inner_lexer.spanned().peekable())
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = (Result<Tokens, LexingError>, Span);
  fn next(&mut self) -> Option<Self::Item> {
    self.0.next()
  }
}

impl<'a> Lexer<'a> {
  pub fn peek(&mut self) -> Option<&(Result<Tokens, LexingError>, Span)> {
    self.0.peek()
  }
}
