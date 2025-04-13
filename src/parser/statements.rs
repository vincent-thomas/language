use crate::lexer::{Lexer, Tokens};

use super::Parseable;

mod function;
mod r#loop;
mod var;

pub(super) enum Statement {
  //Func(function::FunctionStatement),
  VariableDeclaration(var::VariableDeclarationStatement),
  LoopStatement(r#loop::LoopStatement),
}

impl Statement {
  pub fn loop_(lexer: &mut Lexer) -> Self {
    Self::LoopStatement(r#loop::LoopStatement::parse_with_lexer(lexer))
  }

  pub fn var(lexer: &mut Lexer) -> Self {
    Self::VariableDeclaration(
      var::VariableDeclarationStatement::parse_with_lexer(lexer),
    )
  }
  //pub fn func(lexer: &mut Lexer) -> Self {
  //  Self::Func(function::FunctionStatement::parse_with_lexer(lexer))
  //}
}
