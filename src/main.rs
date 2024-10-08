mod ast_parser;
mod lexer;
mod location;
mod parser;
mod result;
mod tokens;

use crate::{lexer::Lexer, parser::Parser, result::AppResult};

fn main() -> AppResult<()> {
    let input = "3 + 4 * (2 + 1)^2";
    dbg!(input);
    let mut lexer = Lexer::new(input);
    dbg!(&lexer);

    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token()? {
        tokens.push(token);
    }
    dbg!(&lexer, &tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    dbg!(&ast);
    println!("AST: {}", ast);
    Ok(())
}
