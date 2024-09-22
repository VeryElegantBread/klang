use crate::misc::handle_text;

pub fn say(line: String, line_num: usize) {
    println!("{}", handle_text::handle_text(line, line_num, 4))
}