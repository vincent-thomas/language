use crate::lexer::{Lexer, LexingError, Tokens};

use super::Parseable;

mod class;
mod function;

#[derive(Debug)]
pub enum Item {
  FunctionItem(function::FunctionItem),
  ClassItem(class::ClassItem),
}

impl Item {
  pub fn parse(lexer: &mut Lexer) -> Option<Result<Item, LexingError>> {
    let token = match &lexer.peek()?.0 {
      Ok(value) => value,
      Err(err) => return Some(Err(err.clone())),
    };

    // Ok: TODO remove to support errors.
    let value = Ok(match token {
      Tokens::Fn => Self::func(lexer),
      Tokens::Class => Self::class(lexer),
      _ => todo!(),
    });

    Some(value)
  }
  //pub fn loop_(lexer: &mut Lexer) -> Self {
  //  Self::LoopStatement(r#loop::LoopStatement::parse_with_lexer(lexer))
  //}
  //
  //pub fn var(lexer: &mut Lexer) -> Self {
  //  Self::VariableDeclaration(
  //    var::VariableDeclarationStatement::parse_with_lexer(lexer),
  //  )
  //}
  pub fn func(lexer: &mut Lexer) -> Self {
    Self::FunctionItem(function::FunctionItem::parse_with_lexer(lexer))
  }

  pub fn class(lexer: &mut Lexer) -> Self {
    Self::ClassItem(class::ClassItem::parse_with_lexer(lexer))
  }
}
