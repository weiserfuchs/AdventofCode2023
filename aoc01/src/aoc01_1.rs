use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    let input = File::open("aoc01.txt").expect("Unable to open file");
    let buffered = BufReader::new(input);
    let mut sum = 0;
    for line in buffered.lines() {
        let s_line = line.unwrap();
        if s_line.len() > 0{
            let mut numbers = "".to_owned();
            for c in s_line.chars(){
                if c.is_digit(10){
                    numbers.push(c)
                }
            }
            //println!("{numbers}");
            let twodigit:i32;
            if numbers.len() == 1{
                let mut tmpstring = String::from("");
                tmpstring.push_str(numbers.as_str());
                tmpstring.push_str(numbers.as_str());
                twodigit = tmpstring.parse().unwrap()
            }else if numbers.len() == 2 {
                twodigit = numbers.parse().unwrap()
            }else{
                let mut tmpstring = String::from("");
                let c1_option = numbers.chars().nth(0);
                let c2_option = numbers.chars().nth_back(0);
                let c1 = c1_option.unwrap_or('0');
                let c2 = c2_option.unwrap_or('0');
                tmpstring.push(c1);
                tmpstring.push(c2);
                twodigit = tmpstring.parse().unwrap()
            }
            sum += twodigit;
            //println!("{twodigit}");
        }
        
    }
    println!("{sum}");
}

