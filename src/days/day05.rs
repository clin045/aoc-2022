use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use regex::Regex;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Instruction{
    qty: u64,
    src: usize,
    dst: usize
}

pub fn solve() -> SolutionPair {
    let (mut stacks, instructions) = read_input();
    run_instructions(&mut stacks, &instructions);
    // Your solution here...
    let sol1: String = output_stacks(&mut stacks);
    let (mut stacks, instructions) = read_input();
    run_instructions_p2(&mut stacks, &instructions);
    let sol2: String = output_stacks(&mut stacks);


    (Solution::Str(sol1), Solution::Str(sol2))
}

fn run_instructions(stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>){
    for i in instructions {
        let mut tmp: Vec<char> = vec![];
        for _ in 0..i.qty{
            tmp.push(stacks[i.src - 1].pop().unwrap());
        }
        stacks[i.dst - 1].append(&mut tmp);
    }
}

fn run_instructions_p2(stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>){
    for i in instructions {
        let mut tmp: Vec<char> = vec![];
        for _ in 0..i.qty{
            tmp.push(stacks[i.src - 1].pop().unwrap());
        }
        tmp.reverse();
        stacks[i.dst - 1].append(&mut tmp);
    }
}

fn output_stacks(stacks: &mut Vec<Vec<char>>) -> String{
    let mut output_vec: Vec<char> = vec![];
    for s in stacks {
        output_vec.push(s.last().unwrap().clone())
    }
    let out_str: String = output_vec.iter().collect();
    out_str
}

fn read_input() -> (Vec<Vec<char>>,Vec<Instruction>) {
    let raw_input = read_to_string("input/05.txt").unwrap();
    let stacks_str = raw_input.split("\n\n").nth(0).unwrap();
    let instructions_str = raw_input.split("\n\n").nth(1).unwrap();
    let instructions_split: Vec<&str> = instructions_str.split("\n").collect();
    // Parse stacks
    let mut stacks_split: Vec<&str> = stacks_str.split("\n").collect();
    let n_stacks = stacks_split.last().unwrap().split("   ").count();
    stacks_split.pop();
    let mut stacks: Vec<Vec<char>> = (0..n_stacks).map(|_| Vec::new()).collect();
    for i in stacks_split.iter().rev(){
        for (char_idx, c) in i.chars().enumerate(){
            if char_idx != 0 && (char_idx - 1) % 4 == 0 {
                if c != ' ' {
                    stacks[(char_idx - 1) / 4].push(c);
                }
            }
        }
    }
    let mut instructions = vec![];
    let re = Regex::new(r"move ([0-9]*?) from ([0-9]*?) to ([0-9]*?)$").unwrap();
    for ins in instructions_split.iter() {
        let caps = re.captures(ins).unwrap();
        let new_ins = Instruction{
            qty: caps.get(1).map_or(0, |m| m.as_str().parse().unwrap()),
            src: caps.get(2).map_or(0, |m| m.as_str().parse().unwrap()),
            dst: caps.get(3).map_or(0, |m| m.as_str().parse().unwrap())
        };
        instructions.push(new_ins);
    }

    (stacks, instructions)
}