use std::io::prelude::*;
use std::fs::File;

pub fn get_input(day: u8, year: u16) -> String {
    let file_name = format!(".AoC-{:04}-{:02}.tmp", year, day);
    let file = File::open(file_name);
    let mut result = String::new();
    if let Ok(mut f) = file {
        f.read_to_string(&mut result).expect("Unable to read file");
    } else {
        unimplemented!()
    }
    return result;
}
