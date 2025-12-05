use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day05(file_loc: &str) {
    let inputs = get_input_arr(file_loc).unwrap();
    println!("---Day  5---");
    println!("Part 1: {}", p1(&inputs));
    println!("Part 2: {}", p2(&inputs));

}

fn get_input_arr<P>(filename: P) -> io::Result<(Vec<(i64, i64)>, Vec<i64>)>
    where P: AsRef<Path> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        let mut fresh = Vec::new();
        let mut ingredients = Vec::new();
        let mut mode = false;

        for line in reader.lines(){
            let line = line?;
            if line == "" {
                mode = true;
                continue
            }
            if mode {
                ingredients.push(line.parse::<i64>().unwrap());
            } else {
                let parsed: Vec<i64> = line.split("-").map(|x| x.parse::<i64>().unwrap()).collect();
                fresh.push((parsed[0], parsed[1]));
            }
        }
        Ok((fresh, ingredients))
    }

fn p1(input: &(Vec<(i64, i64)>, Vec<i64>)) -> i32 {
    let mut count = 0;
    for ingredient in &input.1 {
        for range in &input.0 {
            if ingredient >= &range.0 && ingredient <= &range.1 {
                count += 1;
                break;
            }
        }
    }
    return count
}

fn p2(input: &(Vec<(i64, i64)>, Vec<i64>)) -> i64 {
    let mut combined_ranges: Vec<(i64, i64)> = Vec::new(); 
    let mut starts = Vec::new();
    let mut ends = Vec::new();
    let mut i = 0;
    for range in &input.0{
        starts.push(range.0);
        ends.push(range.1);
    }
    starts.sort();
    ends.sort();
    while i < starts.len(){
        let start_i = i;
        let curr_start = &starts[i];
        let mut try_end = &ends[i];
        let mut moved = true;
        while moved {
            moved = false;
            while i < starts.len() && &starts[i] <= try_end{
                i += 1;
                moved = true;
            }
            try_end = &ends[i-1]
        }
        combined_ranges.push((*curr_start, *try_end));
        if i == start_i {
            i += 1;
        }
    }
    combined_ranges.into_iter().map(
        |(x, y)| y - x + 1
    ).sum()
}