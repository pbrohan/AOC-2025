use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use fancy_regex::Regex;


fn invalid_ids_1(start: i64, end: i64) -> Vec<i64> {
    // invalid ids have even lengths
    let start_length = (start as f64).log10().floor() as i64;
    let end_length = (end as f64).log10().floor() as i64;
    if (start_length % 2 == 0) && (end_length % 2 == 0) && (end_length - start_length == 0) {
        // The wrong length
        return Vec::new()
    } else {
        let mut outs = Vec::new();
        let mut curr = start;
        while curr <= end {
            let next_double = get_next_double(curr);
            if next_double <= end {
                outs.push(next_double);
            }
            curr = next_double + 1;
        }
        return outs
    }
}

fn get_next_double(x: i64) -> i64 {
    let x_len: u32 = (x as f64).log10().floor() as u32 + 1;
    let mut attempt;
    let mut x_start: i64;
    let mut start_len: u32;
    if x_len % 2 == 0 {
        x_start = (x as f64 / 10.0_f64.powf((x_len/2).into())).floor() as i64;
        start_len = x_len/2;
    } else {
        x_start = 10_i64.pow((x_len+1)/2 - 1);
        start_len = (x_len+1)/2;
    }
    attempt = x_start * (10_i64.pow(start_len) + 1);
    if attempt >= x {
        return attempt
    } else {
        x_start += 1;
        start_len = (x_start as f64).log10().floor() as u32 + 1;
        attempt = x_start * (10_i64.pow(start_len) + 1);
    }
    return attempt
}

fn get_input_arr<P>(filename: P) -> io::Result<Vec<(i64, i64)>>
    where P: AsRef<Path> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        let mut out: Vec<(i64, i64)> = Vec::new();


        for line in reader.lines() {
            let line = line?;
            // There's a more rust way to do this
            for pair in line.split(",").collect::<Vec<&str>>(){
                let nums: Vec<&str> = pair.split("-").collect();
                if nums.len() > 1{
                    out.push((nums[0].parse().unwrap_or(0), nums[1].parse().unwrap_or(0)))
                }
            }
            break
        }
    Ok(out)
}
// I do realise that you can do this much faster just using regex
// but I have chosen to do maths I guess!


// Fine, p2 is too annoying not to use regex
fn p2(a: i64, b: i64) -> Vec<i64> {
    let mut curr = a;
    let mut out = Vec::<i64>::new();
    let re = Regex::new(r"^(.*)\1+$").unwrap();
    while curr <= b {
        let currstr = curr.to_string();
        if re.is_match(&currstr).unwrap() {
            out.push(curr);
        }
        curr += 1;
    }
    out
}


pub fn day02(file_loc: &str) {
    let inputs = get_input_arr(file_loc).unwrap();
    let mut p1_sum = 0;
    let mut p2_sum = 0;
    for pair in inputs {
        p1_sum += invalid_ids_1(pair.0, pair.1).iter().sum::<i64>();
        p2_sum += p2(pair.0, pair.1).iter().sum::<i64>();
    }
    println!("---Day  2---");
    println!("Part 1: {}", p1_sum);
    println!("Part 2: {}", p2_sum);
}