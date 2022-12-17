use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

struct DirNode {
    idx: usize,
    name: String,
    filesize: u64,
    children: Vec<usize>,
    parent: Option<usize>
}

struct FileTree{
    tree: Vec<DirNode>
}
impl FileTree{
    fn new() -> FileTree {
        FileTree{tree: Vec::<DirNode>::new()}
    }
    fn create_node(&mut self, parent: usize, name: String) -> usize {
        let new_idx = self.tree.len();
        let new_node = DirNode{
            idx: new_idx,
            name: name,
            filesize: 0,
            children: Vec::<usize>::new(),
            parent: None
        };
        self.tree.push(new_node);
        let parent_node = self.tree.get_mut(parent).unwrap();
        parent_node.children.push(new_idx);
        new_idx
    }
}

pub fn solve() -> SolutionPair {
    println!("{:?}", read_input());
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn read_input() -> Vec<String> {
    let raw_input = read_to_string("input/07_test.txt").unwrap();
    let line_split = raw_input.split("\n").map(|x| x.to_owned()).collect::<Vec<String>>();
    line_split
}

fn build_filetree(commands: Vec<String>) -> FileTree{
    let mut fs_tree = FileTree::new();
    let mut current: usize = 0;
    for cmd in commands {
        let cmd_split: Vec<&str> = cmd.split_whitespace().collect();
        match cmd_split[1] {
            "cd" => {
                let dst = cmd_split[2];
                match dst {
                    "/" => {
                        current = 0;
                    },
                    ".." => {
                        let parent_idx = fs_tree.tree.get(current).unwrap().parent.unwrap();
                        current = parent_idx;
                    }
                    _ => {
                        let child_names: Vec<(String, usize)> = fs_tree.tree.get(current).unwrap()
                            .children
                            .iter()
                            .map(|child| {
                                let name = fs_tree.tree.get(*child).unwrap().name.to_owned();
                                (name, *child) 
                            }).collect();
                        current = child_names.iter().find(|x| {
                            x.0 == dst
                        }).unwrap().1;
                    }
                }
            },
            "ls" => {},
            _ => {
                match cmd_split[0] {
                    "dir" => {
                        fs_tree.create_node(current, cmd_split[1].to_owned());
                    },
                    _ => {
                        let filesize = cmd_split[0].parse::<u64>().unwrap();
                        let cur_node = fs_tree.tree.get_mut(current).unwrap();
                        cur_node.filesize += filesize;
                    }
                }
            }
        }
    }
    todo!();
}