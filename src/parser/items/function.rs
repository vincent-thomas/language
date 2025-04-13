use crate::{
  lexer::{Lexer, Tokens},
  parser::{Ident, Parseable, ReturnType, expressions::BlockExpression},
};

#[derive(Debug)]
pub(super) struct FunctionItem {
  ident: Ident,
  // TODO: params
  return_type: ReturnType,

  block: BlockExpression,
}

impl<'a> Parseable<'a> for FunctionItem {
  fn parse_with_lexer(lexer: &mut Lexer<'a>) -> Self {
    assert_eq!(lexer.next().unwrap().0.unwrap(), Tokens::Fn);

    let fn_ident = lexer.next().unwrap().0.unwrap();

    let Tokens::Ident(ident) = fn_ident else {
      panic!("expected ident");
    };

    assert_eq!(lexer.next().unwrap().0.unwrap(), Tokens::ParamOpen);
    assert_eq!(lexer.next().unwrap().0.unwrap(), Tokens::ParamClose);

    let block = BlockExpression::parse_with_lexer(lexer);

    FunctionItem { ident, return_type: ReturnType(None), block }
  }
}
