use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::BTreeMap;

pub fn main() {
    let input = File::open("aoc02.txt").expect("Unable to open file");
    let buffered = BufReader::new(input);
    let mut sum = 0;
    for line in buffered.lines() {
        let mut scores:BTreeMap<String,i32> = BTreeMap::new();
        
        scores.insert(String::from("red"), 12);
        scores.insert(String::from("green"), 13);
        scores.insert(String::from("blue"), 14);
        
        let s_line = line.unwrap();
        if s_line.len() > 0{
            let mut game_line = s_line.split(":");
            let game_id = game_line.nth(0).unwrap();
            let game_id:i32 = game_id.replace("Game ", "").parse().unwrap_or(0);
            if game_id > 0 {
                let mut b_possible = true;
                let game = game_line.last().unwrap_or("").split(";");
                for g in game {
                    let round = g.split(",");
                    for r in round{
                        let mut color = r.trim().split(" ");
                        let count:i32 = color.nth(0).unwrap_or("0").parse().unwrap_or(0);
                        
                        let mut kv = scores.get_key_value(color.last().unwrap()).unwrap();
                        let a = kv.1 - count;
                        kv.1 = &a;
                        if a < 0 {
                            b_possible = false;
                            //println!("Panic {a}")
                        }
                        //scores.insert(kv.0.to_string(), *kv.1);
                        
                    }
                    
                }
                if b_possible {
                    sum = sum + game_id;
                    //println!("{game_id}")
                }
                
            }
        }
    }
    println!("{sum}");
}