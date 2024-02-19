const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
use std::fmt;

use eframe::egui::plot::Line;

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

#[derive(Debug)]
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

        let mut last_was_op = false;

        for i in 0..elements.len(){
            println!("value={} index={i}", elements[i]);
            if i < elements.len()-1{
                if DIGITS.iter().any(|&x| x == elements[i]){
                    buffer.push(elements[i]);
                    last_was_op= false;
                
                }else if( 
                    i==0 && elements[i]=='-' || //if the first element is a -
                    last_was_op && elements[i]=='-' || //if the element following an operator is -
                    elements[i] == '.' && !buffer.iter().any(|&x| x == '.')  // if the element is . and is'nt already in the buffer (bcuz you can't have 2 . in a f32)
                )   {
                    buffer.push(elements[i]);

                } else {
                    if buffer.len() != 0 {
                        let buffer_result : String = buffer.iter().cloned().collect();
                        buffer = vec![];
                        tokens.push(Token::Atom(buffer_result));
                    }

                    tokens.push(Token::Op(elements[i]));
                    //if elements[i]!='(' && elements[i]!=')'{
                    //    last_was_op = true;
                    //}
                    last_was_op = true
                }
            }else if DIGITS.iter().any(|&x| x == elements[i]){
                buffer.push(elements[i]);
                let buffer_result : String = buffer.iter().cloned().collect();
                buffer = vec![];
                tokens.push(Token::Atom(buffer_result));

            }   else {
                tokens.push(Token::Op(elements[i]));
                last_was_op = true;
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
    println!("{:?}", lexer);
    expr_bp(&mut lexer, 0)
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> S { 
    let mut lhs = match lexer.next() {
        Token::Atom(it) => S::Atom(it),
        Token::Op('(') => {
            let lhs = expr_bp(lexer, 0);
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs
        },
        t => panic!("bad token: {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }

            lexer.next();

            lhs = if op == '?' {
                let mhs = expr_bp(lexer, 0);
                assert_eq!(lexer.next(), Token::Op(':'));
                let rhs = expr_bp(lexer, r_bp);
                S::Cons(op, vec![lhs, mhs, rhs])
            } else {
                let rhs = expr_bp(lexer, r_bp);
                S::Cons(op, vec![lhs, rhs])
            };
            continue;
        }
        break;
    }

    lhs
}

fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    let res = match op {
        '+' | '-' => (1, 2),
        '*' | '/' => (3, 4),
        _ => return None,
    };
    Some(res)
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

pub fn big_brain_calculator(s:S, limit:u32) -> Result<f32, bool>{
    match s {
        S::Atom(val) => {
            println!("{val}");
            return Ok(val.parse::<f32>().unwrap());},
        S::Cons(op, vec) => {match op{
                '+' => return Ok(big_brain_calculator(vec[0].clone(), limit).unwrap() + big_brain_calculator(vec[1].clone(), limit).unwrap()),
                '-' => return Ok(big_brain_calculator(vec[0].clone(), limit).unwrap() - big_brain_calculator(vec[1].clone(), limit).unwrap()),
                '*' => return Ok(big_brain_calculator(vec[0].clone(), limit).unwrap() * big_brain_calculator(vec[1].clone(), limit).unwrap()),
                '/' => {
                    if big_brain_calculator(vec[1].clone(), limit).unwrap()==0.0 {
                        return Err(true);
                    } else {
                        return Ok(big_brain_calculator(vec[0].clone(), limit).unwrap() / big_brain_calculator(vec[1].clone(), limit).unwrap());
                    }
                },
                _ => panic!("unxepected operator {op}")
            }
        } // (x, y)

    }
}

pub fn split_the_lines(mut line:Vec<(f32, f32)>, skips:Vec<f32>)->Vec<Vec<(f32, f32)>>{
    if skips.len()==0{
        return vec![line];
    }

    let mut lines:Vec<Vec<(f32, f32)>> = vec![];
    for i in 0..skips.len(){
        for j in 0..line.len(){
            if skips[i]==line[j].0{
                lines.push(line[0..j].to_vec());
                line.drain(0..=j);
                break;
            }
        }
    }
    if line.len()>0{
        lines.push(line);
    }
    return lines;
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
    fn lexer(){
        let mut lexer = Lexer::new("1+2*3");
        assert_eq!(lexer.tokens, vec![Token::Atom("3".to_owned()), Token::Op('*'), Token::Atom("2".to_owned()), Token::Op('+'), Token::Atom("1".to_owned())])
    }
    #[test]
    fn expr_test() {
        println!("hey");
        let s = expr("1 + 2 * 3");
        println!("{:?}",s);
        assert_eq!(s.to_string(), "(+ 1 (* 2 3))")
    }
    #[test]
    fn line_slicing(){
        let line = vec![(-2.0, 1.0), (-1.0, 2.0), (0.0, 3.0), (1.0, 6.0), (2.0, 1.0), (3.0, 8.0), (4.0, 1.0), (5.0, 7.0), (6.0, 1.0)];
        let skips = vec![-1.0, 3.0];
        let lines = split_the_lines(line, skips);
        assert_eq!(lines, vec![vec![(-2.0, 1.0)], vec![(0.0, 3.0), (1.0, 6.0), (2.0, 1.0)], vec![(4.0, 1.0), (5.0, 7.0), (6.0, 1.0)]])
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