use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn find_keys_for_value<'a>(map: &'a HashMap<&str, &str>, value: &'a str) -> Vec<&'a&'a str> {
    map.iter()
        .filter_map(|(key, &val)| if val == value { Some(key) } else { None })
        .collect()
}
