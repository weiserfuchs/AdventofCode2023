use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::BTreeMap;

pub fn main() {
    let input = File::open("aoc01.txt").expect("Unable to open file");
    let buffered = BufReader::new(input);
    let mut sum: i32 = 0;
    let arr:[&str;9] = ["one,o1ne","two,t2wo","three,t3hree","four,f4our","five,f5ive","six,s6ix","seven,s7even","eight,e8ight","nine,n9ine"];
    for line in buffered.lines() {
        let mut s_line: String = line.unwrap();
        if s_line.len() > 0{
            let mut numbers: String = String::from("");
            if !(s_line.chars().nth(0).unwrap_or('a').is_digit(10) && s_line.chars().nth_back(0).unwrap_or('a').is_digit(10)) {
                let mut scores:BTreeMap<usize,String> = BTreeMap::new();
                //println!("lin: {s_line}");
                for val in arr {
                    let mut s: std::str::Split<'_, &str> = val.split(",");
                    let g_index: Option<usize> = s_line.find(s.nth(0).unwrap());
                    if g_index.is_some() {
                        let i: usize = g_index.unwrap();
                        //println!("i: {i} val: {val}");
                        scores.insert(i,String::from(val));    
                    }
                }
                
                for val in scores  {
                    let v = val.1;
                    let mut s: std::str::Split<'_, &str> = v.split(",");
                    let sf = s.nth(0).unwrap();
                    let sl: &str = s.last().unwrap();
                    //println!("v: {v} sf: {sf} sl: {sl}");
                    s_line = s_line.replace(sf,sl);
                }
            }
            
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
                if c1 == '0' {
                    println!("c1 == 0");
                    println!("lin: {s_line}");
                    println!("num: {numbers}");
                }
                if c2 == '0' {
                    println!("c2 == 0");
                    println!("lin: {s_line}");
                    println!("num: {numbers}");
                }
                tmpstring.push(c1);
                tmpstring.push(c2);
                twodigit = tmpstring.parse().unwrap()
            }
            sum += twodigit;
            //println!("two: {twodigit}");
            //println!("num: {numbers}");
            //println!("lin: {s_line}");
            //println!("sum: {sum}");
            //println!("");
        }
        
    }
    println!("{sum}");
}

