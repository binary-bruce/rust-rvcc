use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("invalid number of arguments");
    }

    if let Some(value) = args.last() {
        println!("  .globl main");
        println!("main:");
        println!("  li a0, {}", value.parse::<i32>().unwrap());
        println!("  ret");
    }
}
