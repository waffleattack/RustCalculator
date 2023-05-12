#![allow(dead_code, unused_variables)]
use std::collections::VecDeque;
pub enum TokenizationError {
    NotAscii,
    Empty,
    UnexpectedChar(u32),
}
enum Token {
  Paren(bool),
  Number(i32),
  Operator(Operator)
}
pub fn tokenize(input: &String) -> Result<VecDeque<Token>, TokenizationError> {
    if input.is_empty() {
        return Err(TokenizationError::Empty);
    }

    if !input.is_ascii() {
        return Err(TokenizationError::NotAscii);
    }
    let mut chars: VecDeque<_> = input.chars().collect();

    let mut tokens: VecDeque<Token> = VecDeque::new();

    let mut col_number: u32 = 0;


}
#[derive(Debug,Clone, Copy)]
enum Operator {
  Plus,
  Minus,
  Divide,
  Multiply
}
impl Operator {
  fn eval(&self, a:i32,b:i32) -> i32{
    match self{
      Operator::Plus => a+b,
      Operator::Minus => a-b,
      Operator::Divide => a/b,
      Operator::Multiply => a*b
    }
  }
  fn from_char(char:char) -> Option<Operator>{
    match char{
      '+' => Some(Operator::Plus),
      '-' => Some(Operator::Minus),
      '*' => Some(Operator::Multiply),
      '/' => Some(Operator::Divide),
      _ => None
    }
  }
}
#[derive(Debug,Clone)]
enum Express {
  Number(i32),
  Express(Box<Expression>)
}
impl Express {
  fn eval(self) -> i32{
    match self {
      Express::Number(c) => c,
      Express::Express(c) =>c.eval()
    }
  }
  fn pack(exp : Expression)-> Express{
    Express::Express(Box::from(exp))
  }
}
#[derive(Debug, Clone)]
struct Expression {
  left_num : Express,
  right_num : Express,
  operator : Operator
}

impl Expression {
  fn eval(self) -> i32 {
    self.operator.eval( self.left_num.eval(), self.right_num.eval())
    }
  }

fn read_until_end_of_number(chars: &mut VecDeque<char>, col_number: &mut u32) -> String {
    read_while(
        |char| char.is_ascii_digit() || *char == '.',
        chars,
        col_number,
    )
}

fn read_until_end_of_identifier(chars: &mut VecDeque<char>, col_number: &mut u32) -> String {
    read_while(|char| char.is_ascii_alphabetic(), chars, col_number)
}
fn read_while(
    predicate: fn(char: &char) -> bool,
    chars: &mut VecDeque<char>,
    col_number: &mut u32,
) -> String {
    let mut res = String::new();

    while !chars.is_empty() {
        let peek = chars.front().unwrap();

        if predicate(peek) {
            res.push(chars.pop_front().unwrap());
            *col_number += 1;
        } else {
            break;
        }
    }

    res
}