use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let all_rounds = read_input();
    // Your solution here...

    let winning_plays: HashMap<char, char> = HashMap::from([
        ('A', 'Y'),
        ('B', 'Z'),
        ('C', 'X')
    ]);

    let tying_plays: HashMap<char, char> = HashMap::from([
        ('A', 'X'),
        ('B', 'Y'),
        ('C', 'Z')
    ]);

    let point_values: HashMap<char, u64> = HashMap::from([
        ('X', 1),
        ('Y', 2),
        ('Z', 3)
    ]);
    let sol1: u64 = all_rounds.iter()
        .fold(0, |a, e|{
            a + part1_round_score(e, &winning_plays, &tying_plays, &point_values)
        });
    let sol2: u64 = all_rounds.iter()
    .fold(0, |a, e|{
        a + part2_round_score(e, &winning_plays, &tying_plays, &point_values)
    });

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn read_input() -> Vec<String> {
    let raw_input = read_to_string("input/02.txt").unwrap();
    let line_split = raw_input.split("\n").map(|x| x.to_owned()).collect::<Vec<String>>();
    line_split
}


fn part1_round_score(s: &String, winning_plays: &HashMap<char, char>,tying_plays: &HashMap<char, char>, point_values: &HashMap<char, u64>) -> u64 {
    let p1 = s.chars().nth(0).unwrap();
    let p2 = s.chars().nth(2).unwrap();
    if p2 == winning_plays[&p1] {
        point_values[&p2] + 6
    }
    else if p2 == tying_plays[&p1] {
        point_values[&p2] + 3
    }
    else {
        point_values[&p2]
    }
}

fn part2_round_score(s: &String, winning_plays: &HashMap<char, char>,tying_plays: &HashMap<char, char>, point_values: &HashMap<char, u64>) -> u64 {
    let p1 = s.chars().nth(0).unwrap();
    let outcome = s.chars().nth(2).unwrap();
    let possible = vec!['X', 'Y', 'Z'];
    return match outcome {
        // Lose
        'X' => {
            let play = possible.iter()
                .filter(|x| {
                    **x != winning_plays[&p1] && **x != tying_plays[&p1]
                }).collect::<Vec<&char>>();
            point_values[*(play.get(0).unwrap())]
        },
        // Draw
        'Y' => {
            point_values[&tying_plays[&p1]] + 3
        },
        // Win
        'Z' => {
            point_values[&winning_plays[&p1]] + 6
        },
        _ => {0},
    }
}