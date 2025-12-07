use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::str::Chars;

pub fn day06(file_loc: &str) {
    let inputs = get_input_p1(file_loc).unwrap();
    println!("---Day  6---");
    println!("Part 1: {}", p1(&inputs));
    println!("Part 2: {:?}", p2(&get_input_p2(file_loc).unwrap()));

}

fn p1(input: &Vec<Vec<String>>) -> i64 {
    let input_len = input[0].len();
    let mut out = 0;
    for i in 0..input_len {
        if input[input.len()-1][i] == "*" {
            out += input[0..input.len()-1].into_iter().fold(
                1, |acc, num| acc * num[i].parse::<i64>().unwrap()
            ); 
        } else {
            out += input[0..input.len()-1].into_iter().fold(
                0, |acc, num| acc + num[i].parse::<i64>().unwrap()
            )
        }
    }
    out
}

fn get_input_p1<P>(filename: P) -> io::Result<Vec<Vec<String>>> 
    where P: AsRef<Path> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        let re = Regex::new(r"\s+").unwrap();
        let mut out: Vec<Vec<String>> = Vec::new();
        
        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();
            out.push(re.replace_all(&trimmed, " ").split(" ").map(|x| x.to_string()).collect::<Vec<_>>());
        }
    Ok(out)
}

fn p2(input: &Vec<String>) -> i64 {
    let mut input = input.clone();
    let len = input.len();
    let mut it: Vec<_> = input.iter_mut().map(|x| x.chars()).collect();
    let mut curr_op = ' ';
    let mut temp: i64 = 0;
    let mut out = 0;
    while let Some(op) = it[len-1].next() {
        if op != ' ' { 
            curr_op = op;
            if curr_op == '*' {
                temp = 1;
            } else {
                temp = 0;
            }};
        let num_str= it[0..len-1].iter_mut().map(|x| x.next().unwrap()).collect::<String>();
        if let Ok(num) = num_str.trim().parse::<i64>() {
            if curr_op == '*' {
                temp *= num;
            } else {
                temp += num;
            }
        } else {
            out += temp
        }
    }
    out += temp;
    out
}

fn get_input_p2<P>(filename: P) -> io::Result<Vec<String>>
    where P: AsRef<Path> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        let out = reader.lines().collect();
        out
    }