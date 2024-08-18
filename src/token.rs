#[derive(Debug, PartialEq)]
pub enum Operator {
    PLUS,
    MINUS,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    NUM(usize),
    PUNCT(Operator),
}

impl Token {
    pub fn tokenize(input: &String) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let input_chars = input.chars();
        let mut current = 0;
        for (i, c) in input_chars.enumerate() {
            if c == '+' {
                let number = Token::to_number(&input[current..i]);
                tokens.push(Token::NUM(number));
                tokens.push(Token::PUNCT(Operator::PLUS));

                current = i + 1;
            }

            if c == '-' {
                let number = Token::to_number(&input[current..i]);
                tokens.push(Token::NUM(number));
                tokens.push(Token::PUNCT(Operator::MINUS));

                current = i + 1;
            }
        }

        let last: usize = Token::to_number(&input[current..]);
        tokens.push(Token::NUM(last));

        tokens
    }

    fn to_number(s: &str) -> usize {
        s.trim().parse::<usize>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = " 12 + 34 - 5 ".to_owned();

        let tokens = Token::tokenize(&input);

        assert_eq!(5, tokens.len());
        assert_eq!(Token::NUM(12), tokens[0]);
        assert_eq!(Token::PUNCT(Operator::PLUS), tokens[1]);
        assert_eq!(Token::NUM(34), tokens[2]);
        assert_eq!(Token::PUNCT(Operator::MINUS), tokens[3]);
        assert_eq!(Token::NUM(5), tokens[4]);
    }
}
