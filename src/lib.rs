use std::num::ParseIntError;
use std::result::Result;

#[derive(Debug)]
pub enum Token {
    Plus,
    Minus,
    Division,
    Multiplication,
    Lparanthesis,
    Rparanthesis,
    Float(f64),
    Int(i128),
    Unkown(char),
    Letter(char),
    DoubleQuote,
}



#[derive(Debug, Clone)]
pub struct Lexer {
    pub text: Vec<char>,
    pub pos: usize,
    pub current_char: char,
}
impl Lexer {

    pub fn parse_number(input: &str) -> Result<u8, ParseIntError> {
        input.trim_end().parse()
    }

    pub fn advance(&mut self) {
        if self.pos < self.text.len() {
            self.pos += 1;
        }
    }

    pub fn make_unknown(&mut self) -> Token {
        return Token::Unkown(self.text[self.pos]);
    }

    pub fn make_letter(&mut self) -> Token {
        return Token::Letter(self.text[self.pos]);
    }

    pub fn make_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut dot_count = 0;

        while self.pos < self.text.len() && (self.text[self.pos].is_ascii_digit() || self.text[self.pos] == '.') {
            let c = self.text[self.pos];
            if c == '.' {
                if dot_count == 1 {
                    break;
                }
                dot_count += 1;
            }
            num_str.push(c);
            self.advance();
        }
        if dot_count == 0 {
            let num_str: i128 = num_str.trim_start_matches(' ').trim_end_matches(' ').parse::<i128>().expect("Not a i128 (integer)").into();
            return Token::Int(num_str);
        } else {
            let num_str: f64 = num_str.trim_start_matches(' ').trim_end_matches(' ').parse::<f64>().expect("Not a f64 (float/decimal)").into();
            return Token::Float(num_str);
        }
    }

    pub fn make_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        while self.pos < self.text.len() {
                match self.text[self.pos] {
                ' ' => self.advance(),
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::Multiplication);
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::Division);
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::Lparanthesis);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::Rparanthesis);
                    self.advance();
                }
                '.' => {
                    self.advance();
                }
                '"' => {
                    tokens.push(Token::DoubleQuote);
                    self.advance();
                }
                _ => {
                    if self.text[self.pos].is_ascii_digit() {
                        tokens.push(self.make_number());
                    } else if self.text[self.pos].is_ascii_alphabetic() {
                        tokens.push(self.make_letter());
                        self.advance();
                    } else {
                        panic!("{:?}",self.make_unknown());
                    }
                }
            }
        }
        return tokens.into_iter().collect();
    }
}
