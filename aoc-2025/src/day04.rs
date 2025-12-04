use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day04(file_loc: &str) {
    let inputs = get_input_arr(file_loc).unwrap();
    println!("---Day  4---");
    println!("Part 1: {}", p1(&inputs));
    println!("Part 2: {}", p2(&inputs));

}

fn get_input_arr<P>(filename: P) -> io::Result<Vec<Vec<usize>>>
    where P: AsRef<Path> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        let mut out = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let mut row = Vec::new();
            for space in line.chars().into_iter() {
                if space == '@' {
                    row.push(1);
                } else {
                    row.push(0);
                }
            }
            out.push(row)
        }
        Ok(out)
}

fn p1(input: &Vec<Vec<usize>>) -> i32 {
    let adj_map = get_adjacency(input);
 
    adj_map.into_iter().map(
        |x| x.into_iter().filter(
            |&x| x < 5 && x > 0
        ).count() as i32
    ).sum()
}

fn p2(input: &Vec<Vec<usize>>) -> i32 {
    let mut input = input.clone();
    let i_l = input.len();
    let row_l = input[0].len();
    let mut removed = 0;
    loop {
        let rem_start = removed;
        let adj_map = get_adjacency(&input);
        for y in 0..i_l {
            for x in 0..row_l {
                if adj_map[y][x] < 5 && adj_map[y][x] > 0 {
                    input[y][x] = 0;
                    removed += 1;
                }
            }
        }
        
        if removed == rem_start  {
            break
        }

    }
    removed
}


fn get_adjacency(input: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let i_l = input.len();
    let row_l = input[0].len();
    let mut adj_map: Vec<Vec<usize>> = vec![vec![0; row_l]; i_l];
    let adj8: Vec<(i32, i32)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    for y in 0..i_l {
        for x in 0..row_l {
            if input[y][x] > 0 {
                adj_map[y][x] += 1;
                for dir in &adj8 {
                    let newy: i32 = y as i32 + dir.0;
                    let newx: i32 = x as i32 + dir.1;
                    if newy >=0 && newy < i_l.try_into().unwrap() && newx >= 0 && newx < row_l.try_into().unwrap() {
                        if input[newy as usize][newx as usize] > 0 {
                            adj_map[newy as usize][newx as usize] += 1;
                        }
                    }
                }
            }
        }
    }
    adj_map
}