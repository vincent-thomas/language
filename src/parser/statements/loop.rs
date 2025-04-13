use crate::lexer::{Lexer, Tokens};

use crate::parser::Parseable;
use crate::parser::expressions::BlockExpression;

#[derive(Debug)]
pub struct LoopStatement {
  block: BlockExpression,
}
impl<'a> Parseable<'a> for LoopStatement {
  fn parse_with_lexer(lexer: &mut Lexer<'a>) -> Self {
    //assert_eq!(lexer.next().unwrap().0.unwrap(), Tokens::Loop);
    let block = BlockExpression::parse_with_lexer(lexer);
    LoopStatement { block }
  }
}
