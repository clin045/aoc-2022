use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, collections::HashSet};

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Rucksack{
    left: HashSet<char>,
    right: HashSet<char>
}


pub fn solve() -> SolutionPair {
    let lines = read_input();
    let elf_triples: Vec<Vec<String>> = lines.iter().enumerate().fold(Vec::new(), |mut a, e| {
        if e.0 % 3 == 0 || a.is_empty(){
            a.push(Vec::new());
            a.last_mut().unwrap().push(e.1.to_owned())
        }
        else {
            a.last_mut().unwrap().push(e.1.to_owned());
        }
        a
    });
    
    let sol1: u64 = lines.iter().fold(0, |a, x| {
        a + score_priority(intersect_rucksack(create_rucksack(x)))
    });
    let sol2: u64 = elf_triples
        .iter()
        .map(|group| {
            let a: HashSet<char> = HashSet::from_iter(group[0].chars());
            let b: HashSet<char> = HashSet::from_iter(group[1].chars());
            let c: HashSet<char> = HashSet::from_iter(group[2].chars());
            let ab = a.intersection(&b).map(|x| x.to_owned()).collect::<HashSet<char>>();
            let final_inter = ab.intersection(&c)
                .map(|x| x.clone())
                .collect::<Vec<char>>();
            final_inter.get(0).unwrap().clone()
        })
        .fold(0, |a, e| {
            a + score_priority(e)
        });
        

    (Solution::U64(sol1), Solution::U64(sol2))
}


fn read_input() -> Vec<String> {
    let raw_input = read_to_string("input/03.txt").unwrap();
    let line_split = raw_input.split("\n").map(|x| x.to_owned()).collect::<Vec<String>>();
    line_split
}

fn create_rucksack(s: &String) -> Rucksack{
    let midpoint = s.len() / 2;
    let mut l: HashSet<char> = HashSet::new();
    let mut r: HashSet<char> = HashSet::new();
    for (idx, i) in s.chars().enumerate(){
        if idx < midpoint {
            l.insert(i);
        }
        else {
            r.insert(i);
        }
    }
    Rucksack{left: l, right: r}
}

fn intersect_rucksack(r: Rucksack) -> char {
    let diff = r.left.intersection(&r.right).collect::<Vec<&char>>();
    *(diff.get(0).unwrap()).clone()
}

fn score_priority(c: char) -> u64 {
    let alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    (alpha.chars().position(|x| x == c).unwrap() + 1) as u64
}