use crate::{Solution, SolutionPair};
use std::{fs::read_to_string};
use ndarray::{Array2};


///////////////////////////////////////////////////////////////////////////////

enum Axis {
    I,
    J
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_input();
    let sol1: u64 = input.indexed_iter().map(
        |x| {
            seeable(&input, x.0, Axis::I, 1) ||
            seeable(&input, x.0, Axis::I, -1) ||
            seeable(&input, x.0, Axis::J, 1) ||
            seeable(&input, x.0, Axis::J, -1)
        }
    ).map(|x| x as u64).sum();

    // let sol1: u64 = tmp.iter().sum();
    let sol2:u64 = input.indexed_iter().map(
        |x| {
            seeable2(&input, x.0, Axis::I, 1) *
            seeable2(&input, x.0, Axis::I, -1) *
            seeable2(&input, x.0, Axis::J, 1) *
            seeable2(&input, x.0, Axis::J, -1)
        }
    ).max().unwrap();

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn read_input() -> Array2<u64> {
    
    let raw_input = read_to_string("input/08.txt").unwrap();
    let line_split: Vec<_> = raw_input.split("\n").collect();
    let i_len = line_split.len();
    let j_len = line_split[0].len();
    let mut trees: Array2<u64> = Array2::zeros((i_len, j_len));
    for (i, _) in line_split.iter().enumerate() {
        for (j, char) in line_split[i].chars().enumerate(){
            trees[[i, j]] = char.to_digit(10).unwrap() as u64;
        }
    }

    trees
}

fn seeable(trees: &Array2<u64>, start: (usize, usize), axis: Axis, step: i32) -> bool {
    let start_val = trees[[start.0, start.1]];
    match axis {
        Axis::I => {
            let mut i_counter = start.0 as i32 + step;
            while let Some(checkval) = trees.get([i_counter as usize, start.1]){
                if *checkval >= start_val {return false}
                i_counter += step;
            }
            
        },
        Axis::J => {
            let mut j_counter = start.1 as i32 + step;
            while let Some(checkval) = trees.get([start.0, j_counter as usize]){
                if *checkval >= start_val {return false}
                j_counter += step;
            }
        }
    }
    true
}

fn seeable2(trees: &Array2<u64>, start: (usize, usize), axis: Axis, step: i32) -> u64 {
    let start_val = trees[[start.0, start.1]];
    match axis {
        Axis::I => {
            let mut seen: u64 = 0;
            let mut i_counter = start.0 as i32 + step;
            while let Some(checkval) = trees.get([i_counter as usize, start.1]){
                seen += 1;
                if *checkval >= start_val {break}
                i_counter += step;
            }
            return seen
            
        },
        Axis::J => {
            let mut j_counter = start.1 as i32 + step;
            let mut seen: u64 = 0;
            while let Some(checkval) = trees.get([start.0, j_counter as usize]){
                seen += 1;
                if *checkval >= start_val {break}
                j_counter += step;
            }
            return seen
        }
    }
}