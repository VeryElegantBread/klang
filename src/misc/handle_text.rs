use crate::misc::error;

pub fn handle_text(line_text: String, line_num: usize, offset: usize) -> String {
    let text = &line_text[offset as usize..];
    let chars: Vec<char> = text.chars().collect();
    let mut last_char = 0;
    let mut quotations = false;
    let mut parts = vec![];
    for character_num in 0..text.len() {
        let character = chars[character_num];
        if character == '"' {
            quotations = !quotations;
        } else if character == '+' && chars[character_num - 1] == ' ' && chars[character_num + 1] == ' ' && quotations == false {
            let temp_text = &text[last_char..character_num - 1];
            let temp_text_chars: Vec<char> = temp_text.chars().collect();
            if temp_text_chars[0] == '"' && temp_text_chars[temp_text.len() - 1] == '"' {
                parts.push(temp_text[1..temp_text.len() - 1].to_string());
            } else {
                error::error(line_text.clone(), line_num, vec![(last_char + offset) as u32, (character_num - 2 + offset) as u32], "unknown thing".to_string(), "maybe put it in quotation marks".to_string())
            }
            last_char = character_num + 2;
        }
        if character_num + 1 == text.len() {
            let temp_text = &text[last_char..];
            let temp_text_chars: Vec<char> = temp_text.chars().collect();
            if temp_text_chars[0] == '"' && temp_text_chars[temp_text.len() - 1] == '"' {
                parts.push(temp_text[1..temp_text.len() - 1].to_string());
            } else {
                error::error(line_text.clone(), line_num, vec![(last_char + offset) as u32, (character_num + offset) as u32], "unknown thing".to_string(), "maybe put it in quotation marks".to_string())
            }
        }
    }
    let mut returned_string = "".to_string();
    for i in &parts {
        returned_string.push_str(&i);
    }
    returned_string
}