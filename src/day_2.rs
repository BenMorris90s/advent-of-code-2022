use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
mod line_parser;


static GAME_RESULTS: [(&str, &str); 9] = [
    ("A X", "draw"),
    ("A Y", "win"),
    ("A Z", "loss"),
    ("B X", "loss"),
    ("B Y", "draw"),
    ("B Z", "win"),
    ("C X", "win"),
    ("C Y", "loss"),
    ("C Z", "draw")
];
static GAME_POINTS: [(&str, i32); 3] = [
    ("win", 6),
    ("draw", 3),
    ("loss", 0)
];
static SELECTION_POINTS: [(char, i32); 3] = [
    ('X', 1),
    ('Y', 2),
    ('Z', 3),
];


fn part_1(input: String) -> i32 {
    let selection_points:  HashMap<char, i32> = HashMap::from(SELECTION_POINTS);
    let game_results: HashMap<&str, &str> = HashMap::from(GAME_RESULTS);
    let game_points: HashMap<&str, i32> = HashMap::from(GAME_POINTS);

    let mut total: i32 = 0;
    let mut games_counted: i32 = 0;

    if let Ok(lines) = line_parser::read_lines(input) {
        for line in lines {
            if let Ok(ip) = line {
                let own_move_played: char = ip.chars().nth(2).expect("Failed to parse char.");
                let own_move_score: i32 = selection_points[&own_move_played];

                let game_result: &str = game_results[&ip as &str];
                let game_score: i32 = game_points[game_result];

                total = total + own_move_score + game_score;
                games_counted = games_counted + 1;
            }
        }
    }

    println!("Total score: {}", total);
    println!("Number of games counted: {}", games_counted);

    return total;
}

fn part_2(input: String) -> i32 {

    let mut total: i32 = 0;

    if let Ok(lines) = line_parser::read_lines(input) {
        for line in lines {
            if let Ok(ip) = line {

            }
        }
    }

    println!("Total score: {}", total);

    return total;
}


pub fn run(input: String) {
    part_1(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2_part_1() {
        let result = part_1(String::from("./inputs/day-2-test.txt"));
        assert_eq!(result, 30);
    }
}