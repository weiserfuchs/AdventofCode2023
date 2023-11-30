use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    let input = File::open("aoc01.txt").expect("Unable to open file");
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        let s_line = line.unwrap();
        if s_line.len() > 0{
            
        }else {

        }
    }
}
