use crate::{
  lexer::{Lexer, Tokens},
  parser::{Ident, Parseable},
};

pub(super) struct VariableDeclarationStatement {
  ident: Ident,

  // WTF: how to do????
  value: (),
}
impl<'a> Parseable<'a> for VariableDeclarationStatement {
  fn parse_with_lexer(lexer: &mut Lexer) -> Self {
    let tokens = Vec::default();

    //loop {
    //  if lexer.peek().expect("EOF").0.unwrap() == Tokens::SemiColon {
    //    // Consume ';'
    //    let _ = lexer.next();
    //    break;
    //  }
    //
    //  tokens.push(lexer.next().unwrap().0.unwrap());
    //}

    //assert_eq!(tokens[0], Tokens::Let);

    let Some(Tokens::Ident(text)) = tokens.get(1) else {
      panic!("no ident after 'let' or invalid ident");
    };

    todo!();

    //while let Some((token, range)) = lexer.next() {
    //  tokens.push(token.unwrap());
    //}
    //assert_eq!(lexer.next().unwrap().0.unwrap(), Tokens::Let);
    //
    //let Tokens::Text(ident) = lexer.next().unwrap().0.unwrap() else {
    //  panic!("expected ident");
    //};
    //
    //VariableDeclarationStatement { ident: Ident::new(ident), value: () }
  }
}
