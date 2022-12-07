use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, collections::HashSet};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = find_marker(read_input(),4);
    let sol2: u64 = find_marker(read_input(),14);
    (Solution::U64(sol1), Solution::U64(sol2))
}

fn read_input() -> Vec<char> {
    let raw_input = read_to_string("input/06.txt").unwrap();
    raw_input.chars().collect()
}

fn find_marker(v: Vec<char>, unique_len: usize) -> u64{
    for (idx, _) in v.iter().skip(unique_len - 1).enumerate(){
        let mut h: HashSet<char> = HashSet::new();
        let idx = idx + unique_len - 1;
        for j in 0..unique_len{
            h.insert(v[idx - j]);
        }
        if h.len() == unique_len{
            return (idx + 1) as u64
        }
    }
    return 0
}