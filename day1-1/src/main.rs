use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> () {
    let val = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .fold(0, |acc, res| {
            acc + res.unwrap().parse::<isize>().unwrap()
        });
    println!("{}", val);
    ()
}
