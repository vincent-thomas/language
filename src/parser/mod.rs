mod expressions;
mod items;
mod statements;

use items::Item;
use statements::Statement;

use crate::lexer::Lexer;

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

#[derive(Debug)]
pub struct AST(Vec<Item>);

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

  pub fn parse(mut self) -> AST {
    // Parse global scope
    let mut items: Vec<Item> = Vec::new();

    while let Some(item) = Item::parse(&mut self.lexer) {
      match item {
        Ok(value) => items.push(value),
        Err(err) => todo!("error handling: {:#?}", err),
      };
    }

    AST(items)
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
