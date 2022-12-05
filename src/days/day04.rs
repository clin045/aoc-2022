use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = read_input().iter()
        .fold(0, |a, e| {
            a + (range_contains(&e.0, &e.1) || range_contains(&e.1, &e.0)) as u64
        });
    let sol2: u64 = read_input().iter()
        .fold(0, |a, e| {
            a + (range_contains_p2(&e.0, &e.1) || range_contains_p2(&e.1, &e.0)) as u64
        });

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn read_input() -> Vec<(String,String)> {
    let raw_input = read_to_string("input/04.txt").unwrap();
    let line_split = raw_input.split("\n").map(|x| x.to_owned()).collect::<Vec<String>>();
    line_split
        .iter()
        .map(|x| {
        let split_str = x.split(",").collect::<Vec<&str>>();
        (split_str[0].to_owned(), split_str[1].to_owned())
    })
        .collect()
}

fn range_contains(a: &String, b: &String) -> bool {
    let a_start = a.split("-").nth(0).unwrap().parse::<u64>().unwrap();
    let a_end = a.split("-").nth(1).unwrap().parse::<u64>().unwrap();
    let b_start = b.split("-").nth(0).unwrap().parse::<u64>().unwrap();
    let b_end = b.split("-").nth(1).unwrap().parse::<u64>().unwrap();
    a_start <= b_start && a_end >= b_end
}

fn range_contains_p2(a: &String, b: &String) -> bool {
    let a_start = a.split("-").nth(0).unwrap().parse::<u64>().unwrap();
    let a_end = a.split("-").nth(1).unwrap().parse::<u64>().unwrap();
    let b_start = b.split("-").nth(0).unwrap().parse::<u64>().unwrap();
    let b_end = b.split("-").nth(1).unwrap().parse::<u64>().unwrap();
    (a_start >= b_start && a_start <= b_end) || (a_end >= b_start && a_end <=b_end)
}