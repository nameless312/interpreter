use anyhow::Result;

#[allow(dead_code)]
#[derive(Debug,PartialEq, Eq)]
enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(String),
    Assign,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lsquirly,
    Rsquirly,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,

    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Bang,
    Minus,
    Slash,
    Asterisk,
    Plus,
}

struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes(),
        };
        lex.read_char();
        return lex;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();
        let token = match self.ch {
            b'{' => Token::Lsquirly,
            b'}' => Token::Rsquirly,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            },
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            } 
            b'/' => Token::Slash,
            b'*' => Token::Asterisk,
            b'<' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::LessThanOrEqual
                } else {
                    Token::LessThan
                }
            },
            b'>' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::GreaterThanOrEqual
                } else {
                    Token::GreaterThan
                }
            },
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_identifier();
                return Ok(match ident.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "true" => Token::True,
                    "false" => Token::False,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    _ => Token::Ident(ident),
                });
            },
            b'0'..=b'9' => {
                let number = self.read_number();
                return Ok(Token::Int(number));
            },
            0 => Token::Eof,
            _ => Token::Illegal,
        };
        self.read_char();
        return Ok(token);
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }
        return String::from_utf8(self.input[position..self.position].to_vec()).unwrap();
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return String::from_utf8(self.input[position..self.position].to_vec()).unwrap();
    }
}
    

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use super::{Token, Lexer};
    #[test]
    fn test_next_token() -> Result<()>{
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input.into());
        let tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lsquirly,
            Token::Rsquirly,
            Token::Comma,
            Token::Semicolon,
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            assert_eq!(token, next_token);
        }

        Ok(())
    }

    #[test]
    fn test_next_token_two() -> Result<()>{
        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
        "#;

        let mut lexer = Lexer::new(input.into());
        let tokens = vec![
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::Rparen,
            Token::Lsquirly,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::Rsquirly,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::Lparen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::Rparen,
            Token::Semicolon,
            Token::Eof,       
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            assert_eq!(token, next_token);
        }

        Ok(())
    }

    #[test]
    fn test_next_token_three() -> Result<()>{
        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
              x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            5 == 10;
            5 != 10;
            5 >= 10;
            5 <= 10;
        "#;

        let mut lexer = Lexer::new(input.into());
        let tokens = vec![
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign, 
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::Rparen,
            Token::Lsquirly,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::Rsquirly,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::Lparen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::LessThan,
            Token::Int("10".into()),
            Token::GreaterThan,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::Equal,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::NotEqual,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::GreaterThanOrEqual,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::LessThanOrEqual,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Eof,
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            assert_eq!(token, next_token);
        }

        Ok(())
    }

}
