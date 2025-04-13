use crate::lexer::{Lexer, LexingError, Tokens};

use super::Parseable;

mod function;
mod r#loop;
mod var;

#[derive(Debug)]
pub(super) enum Statement {
  //Func(function::FunctionStatement),
  VariableDeclaration(var::VariableDeclarationStatement),
  LoopStatement(r#loop::LoopStatement),
}

impl Statement {
  pub fn parse(lexer: &mut Lexer) -> Option<Result<Self, LexingError>> {
    let token = match &lexer.peek()?.0 {
      Ok(value) => value,
      Err(err) => return Some(Err(err.clone())),
    };

    // Ok: TODO remove to support errors.
    let value = Ok(match token {
      Tokens::Let => Self::r#let(lexer),
      //Tokens::Class => Self::class(lexer),
      _ => todo!(),
    });

    Some(value)
  }
  pub fn loop_(lexer: &mut Lexer) -> Self {
    Self::LoopStatement(r#loop::LoopStatement::parse_with_lexer(lexer))
  }

  pub fn r#let(lexer: &mut Lexer) -> Self {
    Self::VariableDeclaration(
      var::VariableDeclarationStatement::parse_with_lexer(lexer),
    )
  }
  //pub fn func(lexer: &mut Lexer) -> Self {
  //  Self::Func(function::FunctionStatement::parse_with_lexer(lexer))
  //}
}
