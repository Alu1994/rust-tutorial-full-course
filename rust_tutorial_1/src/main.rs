#![allow(unused)]

use std::cmp::Ordering;
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use rand::Rng;

fn main() {
    println!("What is your name?");    
    
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("failed to read line");
    println!("This is what you typed: {}", name);

    io::stdin().read_line(&mut name).expect("failed to read line");
}
