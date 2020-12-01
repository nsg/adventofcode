use std::io::{self, BufRead};

fn find(lines: Vec<i64>) -> i64 {
    for n1 in lines.iter() {
        for n2 in lines.iter() {
            if n1 + n2 == 2020 {
                return n1 * n2
            }
        }
    }
    0
}

fn main() {
    let lines: Vec<i64> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();


    println!("{}", find(lines))
}
