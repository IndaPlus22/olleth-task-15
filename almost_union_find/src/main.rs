
use std::{io::{stdin, BufRead, Read, self}};

pub struct UnionFind {
    nodes: Vec<Vec<usize>>,
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        let parent: Vec<usize> = (1..size+1).map(|v| v).collect();
        let nodes: Vec<Vec<usize>> = (1..size+1).map(|v| vec![v; 1]).collect();
        println!("{:?}", parent);
        println!("{:?}", nodes);
        UnionFind { nodes: nodes, parent }
    }

    // union the sets that contain num1 and num2
    pub fn union(&mut self, num1: usize, num2: usize) {
        let set1 = self.find(num1);
        let set2 = self.find(num2);

        if set1 == set2 {
            return;
        }

        let sum1 = self.nodes[set1-1].iter().sum::<usize>();
        let sum2 = self.nodes[set2-1].iter().sum::<usize>();

        if sum1 > sum2 {
            self.parent[set2-1] = set1;
            self.nodes[set1-1].push(set2);
            self.nodes[set2-1].retain(|&x| x != num2)
        } else {
            self.parent[set1-1] = set2;
            self.nodes[set2-1].push(set1);
            self.nodes[set1-1].retain(|&x| x != num1)
        }
        
    }

    // move num1 to set containing num2
    pub fn move_val(&mut self, num1: usize, num2: usize) {
        let set1 = self.find(num1);
        let set2 = self.find(num2);

        if set1 == set2 {
            return;
        }

        self.parent[set1-1] = set2;
        self.nodes[set2-1].push(set1);
        self.nodes[set1-1].retain(|&x| x != num1);
    }

    // returns (size of set containing num, sum of numbers in set containing num)
    pub fn get(&mut self, num: usize) -> (usize, usize) {
        let node = &self.nodes[self.parent[num-1]-1];
        (node.len(), node.iter().sum())
        // (self.sets[set].len(), self.sets[set].iter().sum())
    }

    // returns set of num by finding the root.
    fn find(&mut self, num: usize) -> usize {
        // if self.parent[num-1] != num {
        //     self.parent[num-1] = self.find(num-1);
        // }
        println!("{:?}", self.parent);
        println!("{:?}", self.nodes);
        if !self.nodes[self.parent[num-1]-1].contains(&num) {
            self.parent[num-1] = self.find(num-1);
        }

        self.parent[num-1]
    }

    pub fn update(&self, cmd: Vec<usize>) -> UnionFind {
        UnionFind::new(cmd[0])
    }
}

fn parse_line(line: String) -> Vec<usize> {
    return line.split(" ")
        .map(|x| parse_fast(x))
        .collect();
}

fn parse_fast(value: &str) -> usize {
    // Parse with bytes loop.
    let mut result = 0;
    for b in value.bytes() {
        result = 10 * result + ((b as usize) - 48);
    }
    result
}

// fn main() {
//     //Get length of input
//     let mut lines = stdin()
//         .lock()
//         .lines()
//         .map(|line| line.ok().unwrap());
    
//     // First line: n m
//     // n: size of the set
//     // m: number of commands
//     let first_line = parse_line(lines.next().unwrap());
//     let n = first_line[0];
//     let m = first_line[1];

//     let mut union = UnionFind::new(n);
//     for line in lines.take(m) {
//         let cmd = parse_line(line);
//         match cmd[0] {
//             1 => union.union(cmd[1], cmd[2]),
//             2 => union.move_val(cmd[1], cmd[2]),
//             3 => {
//                 let (size, sum) = union.get(cmd[1]);
//                 println!("{} {}", size, sum);
//             }
//             _ => { union = UnionFind::new(cmd[0]) },
//         }
//     }
// }



fn main() {
    let mut union_thing = UnionFind::new(1);
    let mut buf = String::new();
    while io::stdin().read_line(&mut buf).unwrap() != 0 {
        let line = std::mem::take(&mut buf);

        let cmd: Vec<usize> = line
            .trim()
            .split(" ")
            .map(|x| x.parse().expect("Could not parse numbers!"))
            .collect();

        println!("{:?}", cmd);

            match cmd[0] {
                1 => union_thing.union(cmd[1], cmd[2]),
                2 => union_thing.move_val(cmd[1], cmd[2]),
                3 => {
                    let (size, sum) = union_thing.get(cmd[1]);
                    println!("{} {}", size, sum);
                }
                _ => union_thing = UnionFind::new(cmd[0]),
            }
    }     
}