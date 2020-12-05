use std::io::{self, BufRead};

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut count = 0;
    for line in lines {
        let line_parts: Vec<&str> = line.split(|c: char| " :-".contains(c)).collect();
        if let [char_one_pos, char_two_pos, character, _, password] = &line_parts[..] {
            let mut char_count = 0;

            let pos1: usize = char_one_pos.parse().unwrap();
            let pos1_char = password.chars().nth(pos1 - 1).unwrap().to_string();

            let pos2: usize = char_two_pos.parse().unwrap();
            let pos2_char = password.chars().nth(pos2 - 1).unwrap().to_string();

            if character.eq(&pos1_char) {
                char_count += 1;
            }

            if character.eq(&pos2_char) {
                char_count += 1;
            }

            if char_count == 1 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
