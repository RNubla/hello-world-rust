extern crate reqwest;
extern crate read_input;
extern crate curl;

use std::io;
use std::env;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use read_input::prelude::*;
use curl::easy::Easy;

fn main() {
    println!("Hello, world!");

    let username: String = input().msg("Please enter user name: ").get();
    println!("{}",username);

    let mut done = false;

    while !done {
        let input_inside: i32 = input().msg("Enter number from 1-5: ").inside(1..=5).get();
        match input_inside{
            x if x == 1 => {
                println!("Addition");
                let input1: i32 = input().msg("Enter first number to add: ").get();
                let input2: i32 = input().msg("Enter second number to add: ").get();
                print!("{} + {} = ", input1, input2);
                print!("{}\n",calc(input1, input2));
            },
            x if x == 2 => {
                println!("Hypotenus");
                let a: f32 = input().msg("a^2: ").get();
                let b: f32 = input().msg("b^2: ").get();
                print!("c = √{} + {} = ", a, b);
                print!("{:.2}\n", find_hypotenus(a, b));
            },
            x if x == 3 => {
                read_text_file();
                // download();
                // curl_down();
            },
            x if x == 4 => {
                write_to_text_file();
            }
            x if x == 5 => {
                println!("Quitting....");
                done = true;
            }
            _ => unreachable!(),
        }
    }
}

fn calc(a:i32, b:i32) -> i32{
    return a+b
}

fn find_hypotenus(a:f32, b:f32) -> f32{
    let c = (a*a) + (b*b);
    return c.sqrt();
}
fn read_text_file(){
    let contents = fs::read_to_string("poem.txt").expect("File not found");
    println!("{}", contents);
}

fn write_to_text_file(){
    let mut file = File::create("Write.txt").expect("Failed to create file");
    file.write_all("Hello World".as_bytes()).expect("Write Failed");
    println!("Data written to file");
}

// currently not implemented properly
#[allow(dead_code)]
fn download(){
    let mut resp = reqwest::get("https://dl.dropboxusercontent.com/s/0skuwdlhg6q4fol/History%20of%20GIF.gif").expect("request failed");
    let mut out = File::create("Win1064.gif").expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}
// currently not implemented properly
#[allow(dead_code)]
fn curl_down(){
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url("https://www.rust-lang.org/").unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    println!("{:?}", data);
}