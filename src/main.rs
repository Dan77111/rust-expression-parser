use crate::node::Node;
use std::io;

mod node;

fn main() {
    loop {
        println!("Input the expression to be parsed or 'end' to exit");

        let input = read();

        if input == "end" {
            return;
        }

        if &input[0..3] == "fib" {
            let argument = input[3..].trim().parse::<usize>().unwrap_or(1);
            println!("fib({}) = {}", argument, fib(argument));
            continue;
        }

        let root = Node::from_expression(input);

        match root.evaluate() {
            Ok(result) => {
                println!("The tree representing the operation:\n{}", root);

                println!("The entered expression evaluates to: {}", result)
            }
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn read() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().to_string();
}

fn fib(n: usize) -> u128 {
    let mut cache: Vec<u128> = [1, 1].to_vec();

    rec_fib(n, &mut cache)
}

fn rec_fib(n: usize, cache: &mut Vec<u128>) -> u128 {
    match n {
        number => {
            if cache.len() >= number {
                cache[number - 1]
            } else {
                let fib_value = rec_fib(n - 2, cache) + rec_fib(n - 1, cache);
                cache.insert(number - 1, fib_value);

                cache[number - 1]
            }
        }
    }
}
