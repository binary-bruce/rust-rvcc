use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("invalid number of arguments");
    }

    let input = args.last().unwrap();
    let (mut ops, mut numbers) = tokenize(input);

    println!("  .globl main");
    println!("main:");

    println!("  li a0, {}", numbers.pop().unwrap());

    while let Some(number) = numbers.pop() {
        let op = ops.pop().unwrap();
        match op {
            '+' => {
                println!("  addi a0, a0, {}", number);
            }

            '-' => {
                println!("  addi a0, a0, -{}", number);
            }

            _ => panic!("cannot reach here"),
        }
    }

    println!("  ret");
}

fn tokenize(input: &String) -> (Vec<char>, Vec<usize>) {
    let input_chars = input.chars();
    let mut ops: Vec<char> = vec![];
    let mut numbers: Vec<usize> = vec![];
    let mut current = 0;
    for (i, c) in input_chars.enumerate() {
        if c == '+' || c == '-' {
            ops.push(c);

            let number = to_number(&input[current..i]);
            numbers.push(number);
            current = i + 1;
        }
    }

    numbers.push(to_number(&input[current..]));

    ops.reverse();
    numbers.reverse();

    (ops, numbers)
}

fn to_number(s: &str) -> usize {
    s.trim().parse::<usize>().unwrap()
}
