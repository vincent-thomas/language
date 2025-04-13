use crate::lexer::Lexer;

use super::Parseable;

mod class;
mod function;

#[derive(Debug)]
pub enum Item {
  FunctionItem(function::FunctionItem),
  ClassItem(class::ClassItem),
}

impl Item {
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
