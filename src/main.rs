mod utils;
mod day_2;
mod day_3;

const INPUT_DIRECTORY: &str = "./inputs";

fn main() {
    day_2::run(format!("{}/day-2.txt", INPUT_DIRECTORY));
    day_3::run(format!("{}/day-3.txt", INPUT_DIRECTORY));
}
