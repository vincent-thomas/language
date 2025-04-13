use crate::{
  lexer::{Lexer, Tokens},
  parser::{Ident, Parseable},
};

#[derive(Debug)]
pub struct VariableDeclarationStatement {
  ident: Ident,

  // WTF: how to do????
  value: Box<[Tokens]>,
}
impl<'a> Parseable<'a> for VariableDeclarationStatement {
  fn parse_with_lexer(lexer: &mut Lexer) -> Self {
    let mut tokens = Vec::default();

    loop {
      if *lexer.peek().expect("EOF").0.as_ref().unwrap() == Tokens::Semicolon {
        // Consume ';'
        let _ = lexer.next();
        break;
      }

      tokens.push(lexer.next().unwrap().0.unwrap());
    }

    assert_eq!(tokens[0], Tokens::Let);

    let Some(Tokens::Ident(text)) = tokens.get(1) else {
      panic!("no ident after 'let' or invalid ident");
    };

    let Some(Tokens::AssignmentOperator) = tokens.get(2) else {
      panic!("needs to be '='");
    };

    let tesing = &tokens[3..];

    VariableDeclarationStatement { ident: text.clone(), value: tesing.into() }
  }
}
