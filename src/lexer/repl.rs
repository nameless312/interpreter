use anyhow::Result;
use crate::lexer::lexer::{Lexer, Token};
pub struct Repl{}

impl Repl {
    pub fn run() -> Result<()>{
        loop {
            let mut line = String::new();
            _ = std::io::stdin().read_line(&mut line)?;
            let mut lexer = Lexer::new(line.into());

            loop {
                let token = lexer.next_token()?;
                if token == Token::Eof {
                    break;
                }
                println!("{:?}", token);
            }
        }
    }
}
