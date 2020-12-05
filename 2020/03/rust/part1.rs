use std::io::{self, BufRead};

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut x = 0;
    let mut c = 0;
    for line in lines {
        
        let line_c = line.chars().nth(x).unwrap();
        if line_c.eq(&'#') {
            c += 1;
        }

        x += 3;
        let line_width = line.len() - 1;
        if x > line_width {
            x -= line_width + 1;
        }
    }

    println!("{}", c);
}
