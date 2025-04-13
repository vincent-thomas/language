mod expressions;
mod items;
mod statements;

use items::Item;
use statements::Statement;

use crate::lexer::{Lexer, Tokens};

pub struct Parser<'a> {
  lexer: Lexer<'a>,
}

//enum Expression {
//  BlockExpression(BlockExpression),
//}

// TODO:
pub type TypeDec = ();

#[derive(Debug)]
struct ReturnType(Option<TypeDec>);

/// TODO: nice
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Ident {
  ident: Box<str>,
}

impl Ident {
  pub fn new(name: &str) -> Self {
    Self { ident: name.into() }
  }
}

trait Parseable<'a> {
  fn parse_with_lexer(lexer: &mut Lexer<'a>) -> Self;
}

impl<'a> Parser<'a> {
  pub fn new(lexer: Lexer<'a>) -> Self {
    Self { lexer }
  }

  pub fn parse(mut self) -> Vec<Item> {
    // Parse global scope
    let mut testing: Vec<Item> = Vec::new();

    while let Some((token, range)) = self.lexer.peek() {
      match token.clone() {
        Ok(token) => testing.push(self.parse_item(token)),
        Err(err) => {
          panic!("error: {:#?} {:#?}", err, range)
        }
      }
    }
    testing
  }

  fn parse_item(&mut self, token: Tokens) -> Item {
    match token {
      Tokens::Fn => Item::func(&mut self.lexer),
      Tokens::Class => Item::class(&mut self.lexer),
      _ => todo!(),
    }
  }
  //fn walk_based_on_token(&mut self, token: Tokens<'_>) -> Statement {
  //  match token {
  //    Tokens::FnDec => Statement::func(&mut self.lexer),
  //    Tokens::Let => Statement::var(&mut self.lexer),
  //    Tokens::Loop => Statement::loop_(&mut self.lexer),
  //    _ => panic!("unrecognized token: {:#?}", token),
  //  }
  //}

  //fn parse_fn_dec(&mut self) -> FunctionStatement {
  //  assert_eq!(self.lexer.next().unwrap().0.unwrap(), Tokens::FnDec);
  //
  //  let fn_ident = self.lexer.next().unwrap().0.unwrap();
  //
  //  let Tokens::Text(ident) = fn_ident else {
  //    panic!("expected ident");
  //  };
  //
  //  let ident = Ident::new(ident);
  //
  //  assert_eq!(self.lexer.next().unwrap().0.unwrap(), Tokens::ParamOpen);
  //  assert_eq!(self.lexer.next().unwrap().0.unwrap(), Tokens::ParamClose);
  //
  //  let block = self.parse_block();
  //
  //  FunctionStatement { ident, return_type: ReturnType(None), block }
  //}
  //
  //fn parse_variable_declaration(&mut self) -> VariableDeclarationStatement {
  //  assert_eq!(self.lexer.next().unwrap().0.unwrap(), Tokens::Let);
  //
  //  let Tokens::Text(ident) = self.lexer.next().unwrap().0.unwrap() else {
  //    panic!("expected ident");
  //  };
  //
  //  VariableDeclarationStatement { ident: Ident::new(ident), value: () }
  //}
  //
  //fn parse_loop(&mut self) -> LoopStatement {
  //  todo!();
  //}
  //
  //fn parse_block(&mut self) -> BlockExpression {
  //  assert_eq!(self.lexer.next().unwrap().0.unwrap(), Tokens::BlockOpen);
  //
  //  loop {
  //    // TODO: Parse stuff here.
  //
  //    break;
  //  }
  //  assert_eq!(self.lexer.next().unwrap().0.unwrap(), Tokens::BlockClose);
  //  BlockExpression { statements: Vec::default() }
  //}
}
