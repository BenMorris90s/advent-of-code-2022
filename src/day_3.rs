use crate::utils;

fn union_string_characters(string_one: &str , string_two: &str  ) -> char  {
    let mut shared_character = ' ';

    for character in string_one.chars() {
        if string_two.contains(character) {
            shared_character = character;
            break;
        }
    }

    return shared_character;
}

pub fn run(input: String) -> usize {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let mut total: usize = 0;

    if let Ok(lines) = utils::read_lines(input) {
        for line in lines {
            if let Ok(ip) = line {
                let string_size: usize = ip.len();
                let halves: (&str, &str) = ip.split_at(string_size/2);

                let common_item = union_string_characters(halves.0, halves.1);

                let index = alphabet.iter().position(|&r| r == common_item).unwrap() + 1;

                total = total + index;
            }
        }
    }

    println!("Total priority of common items is: {}", total);

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_3() {
        let result = run(String::from("./inputs/day-3-test.txt"));
        assert_eq!(result, 157);
    }
}