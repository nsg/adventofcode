use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn read_file(filename: &str) -> Result<Vec<i64>, Error>{
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    let mut rvec = vec![];
    for line in lines {
        rvec.push(line?.trim().parse().unwrap())
    }
    Ok(rvec)
}

fn main() {
    let lines = read_file("../input").unwrap();
    'outer: for n1 in lines.clone() {
        for n2 in lines.clone() {
            if n1 + n2 == 2020 {
                println!("{}", n1 * n2);
                break 'outer;
            }
        }
    }
}
