use std::io::{self, BufRead};

fn tree_calc(lines: &Vec<String>, down_step: usize, side_step: usize) -> usize {
    let line_width = lines[0].len();
    let max_height = lines.len();

    let mut x = 0;
    let mut c = 0;
    for y in (0..max_height).step_by(down_step) {
        let line_c = lines[y].chars().nth(x).unwrap();
        if line_c.eq(&'#') {
            c += 1;
        }

        x += side_step;
        if x > line_width - 1 {
            x -= line_width;
        }
    }
    c
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let c1 = tree_calc(&lines, 1, 1);
    let c2 = tree_calc(&lines, 1, 3);
    let c3 = tree_calc(&lines, 1, 5);
    let c4 = tree_calc(&lines, 1, 7);
    let c5 = tree_calc(&lines, 2, 1);
    println!("{}", c1 * c2 * c3 * c4 * c5);
}
