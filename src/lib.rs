const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Atom(String),
    Op(char),
    Eof,
}

#[derive(Debug, Clone)]
pub enum S {
    Atom(String),
    Cons(char, Vec<S>),
}

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        let mut elements: Vec<char> = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .collect();
        let mut tokens: Vec<Token> = vec![];
        let mut buffer: Vec<char> = vec![];
        for i in 0..elements.len(){
            if DIGITS.iter().any(|&x| x == elements[i]){
                buffer.push(elements[i]);
            } else if buffer.len() != 0 {
                let buffer_result : String = buffer.iter().cloned().collect();
                buffer = vec![];
                tokens.push(Token::Atom(buffer_result));
            }
        }
        tokens.reverse();
        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }
    fn peek(&mut self) -> Token {
        self.tokens.last().cloned().unwrap_or(Token::Eof)
    }
}



impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(i) => write!(f, "{}", i),
            S::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

pub fn expr(input: &str) -> S {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer, 0)
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> S { 
    let mut lhs = match lexer.next() {
        Token::Atom(it) => S::Atom(it),
        t => panic!("bad token: {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };

        let (l_bp, r_bp) = infix_binding_power(op);
        if l_bp < min_bp { 
            break;
        }

        lexer.next(); 
        let rhs = expr_bp(lexer, r_bp);

        lhs = S::Cons(op, vec![lhs, rhs]); 
    }

    lhs
}

fn infix_binding_power(op: char) -> (u8, u8) {
    match op {
        '+' | '-' => (1, 2),
        '*' | '/' => (3, 4),
        _ => panic!("bad op: {:?}", op),
    }
}        

pub fn replace_x_by(x_value:i32, function:String)->String{
    let mut result : Vec<String> = function.chars()
        .map(|c| c.to_string())
        .collect();
    for i in 0..result.len(){
        if &result[i] == "X" || &result[i] == "x" {
            result[i] = x_value.to_string();
        }
    }
    return String::from(result.concat());
}

pub fn big_brain_calculator(s:S, limit:u32) -> f32{
    match s {
        S::Atom(val) => {
            println!("{val}");
            return val.parse::<f32>().unwrap();},
        S::Cons(op, vec) => {match op{
                '+' => return big_brain_calculator(vec[0].clone(), limit) + big_brain_calculator(vec[1].clone(), limit),
                '-' => return big_brain_calculator(vec[0].clone(), limit) - big_brain_calculator(vec[1].clone(), limit),
                '*' => return big_brain_calculator(vec[0].clone(), limit) * big_brain_calculator(vec[1].clone(), limit),
                '/' => return big_brain_calculator(vec[0].clone(), limit) / big_brain_calculator(vec[1].clone(), limit),
                _ => panic!("unxepected operator {op}")
            }
        }

    }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn expr_test() {
        println!("hey");
        let s = expr("1 + 2 * 3");
        println!("{:?}",s);
        assert_eq!(s.to_string(), "(+ 1 (* 2 3))")
    }
    #[test]
    fn replace_x() {
        let str = replace_x_by(7, String::from("1+2+x"));
        assert_eq!(str, String::from("1+2+7"));
        let str = replace_x_by(-7, String::from("1+2+x"));
        assert_eq!(str, String::from("1+2+-7"));
        let str = replace_x_by(0, String::from("1+2+x"));
        assert_eq!(str, String::from("1+2+0"));
    }
}