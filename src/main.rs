#![allow(dead_code, unused_variables)]


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
  fn eval(exp : Expression)-> Express{
    Express:Express(Box::from(exp))
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
fn main() {
  let expres1 = Expression {
    left_num: Express::Number(2),
    right_num: Express::Number(0),
    operator: Operator::Multiply
  };
  let expres2 = Expression {
    left_num: Express::Express(Box::from(expres1)),
    right_num: Express::Number(4),
    operator: Operator::Plus

  };
  
  println!("{:#?}",expres2);
  println!("{}",expres2.eval())
}