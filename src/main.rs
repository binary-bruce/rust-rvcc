mod token;

use std::env;

use token::{Operator, Token};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("invalid number of arguments");
    }

    let input = args.last().unwrap();
    let mut tokens = Token::tokenize(input);
    tokens.reverse();

    println!("  .globl main");
    println!("main:");

    if let Token::NUM(num) = tokens.pop().unwrap() {
        println!("  li a0, {}", num);
    } else {
        panic!("invalid input");
    }

    while let Some(token) = tokens.pop() {
        if let Some(Token::NUM(num)) = tokens.pop() {
            match token {
                Token::PUNCT(Operator::PLUS) => {
                    println!("  addi a0, a0, {}", num);
                }

                Token::PUNCT(Operator::MINUS) => {
                    println!("  addi a0, a0, -{}", num);
                }

                _ => panic!("invalid input"),
            }
        } else {
            panic!("invalid input");
        }
    }

    println!("  ret");
}
