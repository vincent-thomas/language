use lexer::Lexer;
use parser::Parser;

mod lexer;
mod parser;

struct ProgramBuilder;

struct Program;

fn main() {
  let lexer = Lexer::from_source(
    r#"

    class vdtesting {
      nice string,
      verynice string,
    }
    fn main() {
    let testing = testing;
    }
  "#,
  );

  let result = Parser::new(lexer).parse();

  println!("{:#?}", result);

  //while let Some(item) = gcc
  //let tesul = result.next();

  //for item in result {
  //}
}

#[test]
fn testing() {
  let _result = Lexer::from_source(
    r#"
    fn testing() {
      let nice = "whaat\n"; @@@@@@
      let result = 6 * 9 + -3;
    }
  "#,
  );
}
