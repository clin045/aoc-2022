use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut elftotals = read_input().iter()
        .map(|x| x.iter().sum())
        .collect::<Vec<u64>>();
    
    elftotals.sort();
    elftotals.reverse();

    let sol1 = elftotals
        .iter()
        .max().unwrap().to_owned();

    let sol2: u64 = elftotals
        .get(0..3)
        .unwrap()
        .to_vec()
        .iter().sum::<u64>();

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn read_input() -> Vec<Vec<u64>> {
    let raw_input = read_to_string("input/01.txt").unwrap();
    let line_split = raw_input.split("\n");
    let chunks = line_split.fold(Vec::new(), |mut acc, x|{
        if x == "" || acc.is_empty() {
            acc.push(Vec::new());
        }
        else {
            acc.last_mut().unwrap().push(x.parse::<u64>().unwrap());
        }
        acc
    });
    chunks
}