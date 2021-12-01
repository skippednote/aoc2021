use std::fs;

fn part_a(c: &Vec<i64>) {
    let mut increases = 0;
    for i in 1..c.len() {
        if c[i] > c[i - 1] {
            increases += 1;
        }
    }
    println!("AOC-01: Part A => {}", increases);
}

fn part_b(c: &Vec<i64>) {
    let mut increases = 0;
    for i in 3..c.len() {
        let a = c[i] + c[i - 1] + c[i - 2];
        let b = c[i - 1] + c[i - 2] + c[i - 3];
        if a > b {
            increases += 1;
        }
    }
    println!("AOC-01: Part B => {}", increases);
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read the input file.");
    let c: Vec<i64> = data
        .trim()
        .split("\n")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();

    part_a(&c);
    part_b(&c);
}
