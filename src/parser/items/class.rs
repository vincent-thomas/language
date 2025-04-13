use std::collections::HashMap;

use crate::{
  lexer::{Lexer, Tokens},
  parser::{Ident, Parseable, TypeDec},
};

#[derive(Debug)]
pub(super) struct ClassItem {
  ident: Ident,
  data: HashMap<Ident, TypeDec>, //block: BlockExpression,
}

impl<'a> Parseable<'a> for ClassItem {
  fn parse_with_lexer(lexer: &mut Lexer<'a>) -> Self {
    assert_eq!(lexer.next().unwrap().0.unwrap(), Tokens::Class);

    let Tokens::Ident(ident) = lexer.next().unwrap().0.unwrap() else {
      panic!("expected ident");
    };

    assert_eq!(lexer.next().unwrap().0.unwrap(), Tokens::BlockOpen);

    let mut data_builder = HashMap::default();

    loop {
      if *lexer.peek().unwrap().0.as_ref().unwrap() == Tokens::BlockClose {
        let _ = lexer.next();
        break;
      }

      // Get key
      let Tokens::Ident(ident) = dbg!(lexer.next().unwrap().0.unwrap()) else {
        panic!("is not ident");
      };

      let mut values = Vec::new();

      // Get value, loop because can contain spaces (or??)
      loop {
        if *lexer.peek().unwrap().0.as_ref().unwrap() == Tokens::Comma {
          let _ = lexer.next();
          break;
        }
        let Tokens::Ident(value) = lexer.next().unwrap().0.unwrap() else {
          panic!("is not ident");
        };

        values.push(value);
      }

      data_builder.insert(ident, ());
    }

    Self { data: data_builder, ident }
  }
}
