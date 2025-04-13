pub use block::BlockExpression;

use crate::lexer::{Lexer, LexingError, Tokens};

use super::Parseable;

mod block;
pub(super) enum Expression {
  Block(BlockExpression),
  //VariableDeclaration(var::VariableDeclarationStatement),
  //LoopStatement(r#loop::LoopStatement),
}

impl Expression {
  pub fn parse(lexer: &mut Lexer) -> Option<Result<Expression, LexingError>> {
    let token = match &lexer.peek()?.0 {
      Ok(value) => value,
      Err(err) => return Some(Err(err.clone())),
    };

    // Ok: TEMP
    let result = Ok(match token {
      Tokens::BlockOpen => Self::block(lexer),
      _ => todo!(),
    });

    Some(result)
  }
  //pub fn loop_(lexer: &mut Lexer) -> Self {
  //  Self::LoopStatement(r#loop::LoopStatement::parse_with_lexer(lexer))
  //}

  //  Self::VariableDeclaration(
  //  pub fn var(lexer: &mut Lexer) -> Self {
  //    var::VariableDeclarationStatement::parse_with_lexer(lexer),
  //  )
  //}
  pub fn block(lexer: &mut Lexer) -> Self {
    Self::Block(block::BlockExpression::parse_with_lexer(lexer))
  }
}
