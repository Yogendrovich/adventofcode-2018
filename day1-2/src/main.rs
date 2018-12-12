use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> () {
    let mut observed_frequencies = HashSet::new();
    let parsed: Vec<isize> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|ln| ln.unwrap().parse().unwrap())
        .collect();

    let mut acc: isize = 0;
    observed_frequencies.insert(acc);
    for input in parsed.into_iter().cycle() {
        acc += input;
        if !observed_frequencies.insert(acc) {
            break;
        }
    }

    println!("{}", acc);
    ()
}
