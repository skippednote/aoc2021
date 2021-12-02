use std::fs;

fn part_a(data: &String) {
    let mut x = 0;
    let mut y = 0;

    for line in data.lines() {
        let mut parts = line.split(" ");
        let direction = parts.next().unwrap();
        let amount: i64 = parts.next().unwrap().parse().unwrap();

        match direction {
            "forward" => x += amount,
            "up" => y -= amount,
            "down" => y += amount,
            _ => panic!(),
        }
    }

    println!("AOC-02: Part A => {}", x * y);
}

fn part_b(data: &String) {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for line in data.lines() {
        let mut parts = line.split(" ");
        let direction = parts.next().unwrap();
        let amount: i64 = parts.next().unwrap().parse().unwrap();

        match direction {
            "forward" => {
                x += amount;
                y += aim * amount
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => panic!(),
        }
    }

    println!("AOC-02: Part B => {}", x * y);
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read the input file!");
    part_a(&data);
    part_b(&data);
}
