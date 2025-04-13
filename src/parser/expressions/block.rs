use crate::{
  lexer::{Lexer, Tokens},
  parser::Parseable,
};

#[derive(Debug)]
pub struct BlockExpression {
  statements: Vec<Vec<Tokens>>,
}

impl<'a> Parseable<'a> for BlockExpression {
  fn parse_with_lexer(lexer: &mut Lexer<'a>) -> Self {
    let mut tokens = Vec::new();
    assert_eq!(lexer.next().unwrap().0.unwrap(), Tokens::BlockOpen);

    loop {
      if *lexer.peek().unwrap().0.as_ref().unwrap() == Tokens::BlockClose {
        let _ = lexer.next();
        break;
      }

      let mut current_line_of_tokens = Vec::new();
      loop {
        if *lexer.peek().unwrap().0.as_ref().unwrap() == Tokens::Semicolon {
          // Consume token
          let _ = lexer.next();
          break;
        }

        current_line_of_tokens.push(dbg!(lexer.next().unwrap().0.unwrap()));
      }
      tokens.push(current_line_of_tokens);
    }

    BlockExpression { statements: tokens }
  }
}
