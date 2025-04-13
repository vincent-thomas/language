use crate::{
  lexer::{Lexer, Tokens},
  parser::{Parseable, Statement},
};

#[derive(Debug)]
pub struct BlockExpression {
  statements: Vec<Statement>,
}

impl<'a> Parseable<'a> for BlockExpression {
  fn parse_with_lexer(lexer: &mut Lexer<'a>) -> Self {
    assert_eq!(lexer.next().unwrap().0.unwrap(), Tokens::BlockOpen);

    let mut statements = Vec::default();

    loop {
      if *lexer.peek().unwrap().0.as_ref().unwrap() == Tokens::BlockClose {
        break;
      }

      let statement = Statement::parse(lexer).unwrap().unwrap();
      statements.push(statement);
    }
    assert_eq!(lexer.next().unwrap().0.unwrap(), Tokens::BlockClose);

    BlockExpression { statements }
  }
}
