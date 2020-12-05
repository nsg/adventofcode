use std::io::{self, BufRead};

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut count = 0;
    for line in lines {
        let line_parts: Vec<&str> = line.split(|c: char| " :-".contains(c)).collect();
        if let [range_start, range_stop, character, _, password] = &line_parts[..] {
            let character_count = password.matches(character).count();
            if character_count >= range_start.parse().unwrap() && character_count <= range_stop.parse().unwrap() {
                count += 1
            }
        }
    }
    println!("{}", count)
}
