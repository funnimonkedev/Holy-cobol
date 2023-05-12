use std::num::ParseIntError;
use std::result::Result;

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Division,
    Multiplication,
    Lparanthesis,
    Rparanthesis,
    Float(f64),
    Int(i128),
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

    pub fn make_number(&mut self) -> Operator {
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
            return Operator::Int(num_str);
        } else {
            let num_str: f64 = num_str.trim_start_matches(' ').trim_end_matches(' ').parse::<f64>().expect("Not a f64 (float/decimal)").into();
            return Operator::Float(num_str);
        }
    }

    pub fn make_tokens(&mut self) -> Vec<Operator> {
        let mut tokens: Vec<Operator> = vec![];
        while self.pos < self.text.len() {
                match self.text[self.pos] {
                ' ' => self.advance(),
                '+' => {
                    tokens.push(Operator::Plus);
                    self.advance();
                }
                '-' => {
                    tokens.push(Operator::Minus);
                    self.advance();
                }
                '*' => {
                    tokens.push(Operator::Multiplication);
                    self.advance();
                }
                '/' => {
                    tokens.push(Operator::Division);
                    self.advance();
                }
                '(' => {
                    tokens.push(Operator::Lparanthesis);
                    self.advance();
                }
                ')' => {
                    tokens.push(Operator::Rparanthesis);
                    self.advance();
                }
                '.' => {
                    self.advance();
                }
                _ => {
                    if self.text[self.pos].is_ascii_digit() {
                        tokens.push(self.make_number());
                    } else {
                        self.advance();
                    }
                }
            }
        }
        return tokens.into_iter().collect();
    }
}