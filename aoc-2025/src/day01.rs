use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day01(file_loc: &str) {
    let inputs = get_input_arr(file_loc).unwrap();
    println!("Part 1: {}", inputs.0);
    println!("Part 2: {}", inputs.1);
}

fn get_input_arr<P>(filename: P) -> io::Result<(i32, i32)>
    where P: AsRef<Path> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);

        let mut try_val = 50;
        let mut zeros = 0;
        let mut all_zeros: i32 = 0;

        for line in reader.lines() {
            let line = line?;
            let mut input_chars = line.chars();
            let indicator = input_chars.next();
            let mut converted: i32 = input_chars.as_str().parse().expect("couldn't parse int");
            let temp_start;
            if indicator == Some('L') {
                if try_val != 0 {
                    temp_start = 100 - try_val + converted;
                } else {
                    temp_start = converted;
                }
                converted = -converted;
            } else {
                temp_start = try_val + converted;
            }
            all_zeros += temp_start/100;
            //println!("{} {} {} {}", line, try_val, temp_start, all_zeros);
            try_val = (try_val + converted).rem_euclid(100);
            if try_val == 0 { zeros += 1}

        }
        Ok((zeros, all_zeros))
    }