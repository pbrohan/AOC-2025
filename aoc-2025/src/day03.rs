use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day03(file_loc: &str) {
    let inputs = get_input_arr(file_loc).unwrap();
    println!("---Day  3---");
    println!("Part 1: {}", p1(&inputs));
    println!{"Part 2: {}", p2(&inputs)};

}

fn get_input_arr<P>(filename: P) -> io::Result<Vec<String>>
    where P: AsRef<Path> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        let mut out = Vec::new();

        for line in reader.lines() {
            out.push(line?);
        }
        Ok(out)
    }

fn p1(input: &Vec<String>) -> i64 {
    let p1: i64 = input.into_iter()
        .map(
            |x| 
                {let m = x.split("").max().unwrap();
                let t: Vec<_> = x.splitn(2, m).collect();
                if t[1].len() > 0 {
                    (m.to_owned() + t[1].split("").max().unwrap())
                    .parse::<i64>().unwrap()
                } else {
                    (t[0].split("").max().unwrap().to_owned() + m)
                    .parse::<i64>().unwrap()
                }}

            
        ).sum();
    p1
}

fn p2(input: &Vec<String>) -> i64{
    let p2: i64 = input.into_iter()
        .map(
            |x| {
                largest_subnumber(x, 12).unwrap()
            }
        ).sum();
    p2
}



fn largest_subnumber(input: &str, x: u32) -> Result<i64, &str> {
    if x > input.chars().count().try_into().unwrap() {
       return Err("not enough characters");
    }
    if x == 0 {
        return Err("recursed too far!");
    }
    if x == 1 {
        return Ok(input.split("").max().unwrap().parse::<i64>().unwrap())
    }

    let mut to_check = 9;
    while to_check > -1{
        while !input.contains(&to_check.to_string()) && to_check != -1{
            to_check -= 1;
        }
        let s: Vec<_> = input.splitn(2, &to_check.to_string()).collect();
        if let Ok(sn) = largest_subnumber(s[1], x-1) {
            return Ok(sn + to_check * 10_i64.pow(x-1))
        } else {
            to_check -=1;
        }
    }
    return Err("found nothing")
}