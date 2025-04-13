pub use block::BlockExpression;

use crate::lexer::Lexer;

use super::Parseable;

mod block;
pub(super) enum Expression {
  Block(BlockExpression),
  //VariableDeclaration(var::VariableDeclarationStatement),
  //LoopStatement(r#loop::LoopStatement),
}

impl Expression {
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
