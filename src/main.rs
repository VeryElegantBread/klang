use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
mod misc;
mod keywords;

fn main() -> Result<(), &'static str> {
    let arguments: Vec<_> = env::args().collect();

    // checks if a file was inputed, if it exists, and if it has the right extension
    let file_path = match arguments.len() > 1 {
        true => &arguments[1],
        false => return Err("Please give a file to run")
    };
    let file = File::open(file_path).map_err(|_e| "File does not exist")?;
    if Path::new(&file_path).extension() != Some(OsStr::new("klng")) {
        return Err("Invalid file extention.")
    }

    let lines: Vec<Result<String, io::Error>> = io::BufReader::new(file).lines().collect();
    
    let mut current_line_num = 0;
    loop {
        // gets the current line and removes all comments and stuff
        let Some(Ok(ref base_line)) = lines.get(current_line_num) else { return Ok(()); };
        let line = misc::format::format_line(base_line);

        if line != "" {
            // does the right thing based on the first word
            let word = line.split([' ', '(']).nth(0).unwrap();
            match word {
                "say" => keywords::say::say(line.to_string(), current_line_num),
                _ => misc::error::error(line.to_string(), current_line_num, vec![0, word.len() as u32], format!("Unknown word: {}", word), "use a real word".to_string()),
            }
        }
        current_line_num += 1;
    }
}