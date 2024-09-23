use crate::misc::{error::error, handle_text::handle_text};

pub fn say(line: String, line_num: usize) {
    if line.chars().collect::<Vec<_>>()[3] == '(' && line.chars().collect::<Vec<_>>()[line.len() - 1] == ')' { // if its like this say(...)
        println!("{}", handle_text(line[4..line.len() - 1].to_string(), line, line_num, 4));
    } else {
        error(line.clone(), line_num, vec![3, line.len() as u32], "Thing to print not correctly in parentheses".to_string(), "make sure there is ( after say and ) at the end".to_string());
    }
}