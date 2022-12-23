use std::collections::HashMap;
use crate::utils;



const LOSS: &'static str = "loss";
const WIN: &'static str = "win";
const DRAW: &'static str = "draw";

static GAME_RESULTS: [(&str, &str); 9] = [
    ("A X", DRAW),
    ("A Y", WIN),
    ("A Z", LOSS),
    ("B X", LOSS),
    ("B Y", DRAW),
    ("B Z", WIN),
    ("C X", WIN),
    ("C Y", LOSS),
    ("C Z", DRAW)
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
static GAME_RESULT_CODE_TRANSLATIONS: [(char, &str); 3] = [
    ('X', LOSS),
    ('Y', DRAW),
    ('Z', WIN)
];


fn part_1(input: &String) -> i32 {
    let selection_points:  HashMap<char, i32> = HashMap::from(SELECTION_POINTS);
    let game_results: HashMap<&str, &str> = HashMap::from(GAME_RESULTS);
    let game_points: HashMap<&str, i32> = HashMap::from(GAME_POINTS);

    let mut total: i32 = 0;
    let mut games_counted: i32 = 0;

    if let Ok(lines) = utils::read_lines(input) {
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

    println!("Total score for part 1: {}", total);
    println!("Number of games counted for part 1: {}", games_counted);

    return total;
}

fn part_2(input: &String) -> i32 {
    let game_results: HashMap<&str, &str> = HashMap::from(GAME_RESULTS);
    let game_result_code_translations: HashMap<char, &str> = HashMap::from(GAME_RESULT_CODE_TRANSLATIONS);
    let selection_points:  HashMap<char, i32> = HashMap::from(SELECTION_POINTS);
    let game_points: HashMap<&str, i32> = HashMap::from(GAME_POINTS);


    let mut total: i32 = 0;

    if let Ok(lines) = utils::read_lines(input) {
        for line in lines {
            if let Ok(ip) = line {
                let game_result_code: char = ip.chars().nth(2).expect("Failed to parse char.");
                let game_result: &str = game_result_code_translations[&game_result_code];

                let opponent_choice: char = ip.chars().nth(0).expect("Failed to parse char.");
                let possible_games: Vec<&&str> = utils::find_keys_for_value(&game_results, game_result);

                let mut our_choice_score: i32 = 0;
                for game in possible_games.into_iter() {
                    if game.chars().nth(0).unwrap() == opponent_choice {
                        let our_choice = game.chars().nth(2).unwrap();
                        our_choice_score = selection_points[&our_choice];
                    }
                }

                total = total + game_points[game_result] + our_choice_score;
            }
        }
    }

    println!("Total score for part 2: {}", total);

    return total;
}


pub fn run(input: String) {
    part_1(&input);
    part_2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2_part_1() {
        let result = part_1(&String::from("./inputs/day-2-test.txt"));
        assert_eq!(result, 30);
    }
}