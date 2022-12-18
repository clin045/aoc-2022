use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, collections::HashSet};

///////////////////////////////////////////////////////////////////////////////
enum Direction {L, R, U, D}
struct Motion {
    dir: Direction,
    steps: u64
}
#[derive(Debug)]
struct Rope{
    child: Option<usize>,
    x: i32,
    y: i32
}

struct RopeChain {
    chain: Vec<Rope>
}

pub fn solve() -> SolutionPair {
    //perform_motion(&input[1],&mut chain, &mut visited);
    // Your solution here...
    let sol1: u64 = p1() ;
    let sol2: u64 = p2();

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn p1() -> u64 {
    let input = read_input();
    let tail = Rope{
        child: None,
        x: 0,
        y: 0
    };
    let head = Rope{
        child: Some(1),
        x: 0,
        y: 0
    };
    let mut chain = RopeChain{chain: vec![head, tail]};
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for motion in input {
        perform_motion(&motion, &mut chain, &mut visited);
    }
    visited.len() as u64
}

fn p2() -> u64 {
    let input = read_input();
    let mut chain = RopeChain{chain: vec![]};
    for i in 0..9 {
        chain.chain.push(
            Rope {
                child: Some(i + 1),
                x: 0,
                y: 0
            }
        )
    }
    chain.chain.push(
        Rope {
            child: None,
            x: 0,
            y: 0
        }
    );
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for motion in input {
        perform_motion(&motion, &mut chain, &mut visited);
    }
    visited.len() as u64
}

fn perform_motion(motion: &Motion, chain: &mut RopeChain, visited: &mut HashSet<(i32, i32)>){
    for _ in 0..motion.steps {
        let head = chain.chain.get_mut(0).unwrap();
        match motion.dir {
            Direction::U => {head.y += 1},
            Direction::D => {head.y -= 1},
            Direction::R => {head.x += 1},
            Direction::L => {head.x -= 1}
        }
        propagate(chain, visited);
    }
}

fn propagate(chain: &mut RopeChain, visited: &mut HashSet<(i32, i32)>){
    let mut cur_idx = Some(0);
    let max_len = chain.chain.len();
    let mut dst_coord = (chain.chain.get(0).unwrap().x, chain.chain.get(0).unwrap().y);
    while cur_idx.is_some() {
        let cur_node = chain.chain.get_mut(cur_idx.unwrap()).unwrap();
        // dst_coord = (cur_node.x, cur_node.y);
        let diff = (cur_node.x - dst_coord.0, cur_node.y - dst_coord.1);
        if diff.0.abs() > 1 || diff.1.abs() > 1 {
            if diff.0 > 0 {
                cur_node.x -= 1;
            }
            if diff.0 < 0 {
                cur_node.x += 1
            }
            if diff.1 > 0 {
                cur_node.y -= 1
            }
            if diff.1 < 0 {
                cur_node.y += 1
            }
        }
        if cur_idx.unwrap() == max_len - 1 {
            visited.insert((cur_node.x, cur_node.y));
        }
        println!("{} {}",cur_node.x, cur_node.y);
        cur_idx = cur_node.child;
        dst_coord = (cur_node.x, cur_node.y);
    }
}

fn read_input() ->  Vec<Motion>{
    let raw_input = read_to_string("input/09.txt").unwrap();
    let line_split: Vec<_> = raw_input.split("\n").collect();
    let mut motions: Vec<Motion> = Vec::new();
    for l in line_split.iter(){
        let dir: Option<Direction> = match l.split(" ").nth(0).unwrap() {
                "R" => {Some(Direction::R)},
                "L" => {Some(Direction::L)},
                "U" => {Some(Direction::U)},
                "D" => {Some(Direction::D)},
                _ => {None}
            };
        let steps = l.split(" ").nth(1).unwrap().parse::<u64>().unwrap();
        motions.push(Motion{dir: dir.unwrap(), steps: steps});

    }
    motions
}