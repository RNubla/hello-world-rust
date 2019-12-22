extern crate reqwest;
extern crate read_input;

// use std::io;
// use std::fs::File;
use read_input::prelude::*;

fn main() {
    println!("Hello, world!");

    let username: String = input().msg("Please enter user name:").get();
    println!("{}",username);

    let input_inside: i32 = input().msg("Enter number from 1-3: ").inside(1..=3).get();

    match input_inside{
        x if x == 1 => {
            println!("Addition");
            let input1: i32 = input().msg("Enter first number to add: ").get();
            let input2: i32 = input().msg("Enter second number to add: ").get();
            print!("{} + {} = ", input1, input2);
            print!("{}",calc(input1, input2));
        },
        x if x == 2 => {
            println!("Hypotenus");
            let a: f32 = input().msg("a^2: ").get();
            let b: f32 = input().msg("b^2: ").get();
            print!("√{} + {} = ", a, b);
            print!("{}", find_hypotenus(a, b))
        },
        3 => println!("Quit"),
        _ => unreachable!(),
    }
}

fn calc(a:i32, b:i32) -> i32{
    return a+b
}

fn find_hypotenus(a:f32, b:f32) -> f32{
    let c = (a*a) + (b*b);
    return c.sqrt();
}
