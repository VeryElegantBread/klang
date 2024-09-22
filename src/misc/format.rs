pub fn format_line(line: &String) -> String {
    let mut new_line = line.clone();
    
    //remove spaces from begining
    for character in line.chars() {
        if character == ' '{
            new_line = new_line[1..].to_string();
        } else {
            break;
        }
    }
    let line = new_line.clone();

    //remove comments
    let mut quotes = false;
    let mut slashes = 0;
    for character in line.chars() {
        if slashes == 2 {
            new_line = new_line[..new_line.len() - 1].to_string();
        } else if character == '"' {
            quotes = !quotes;
        } else if character == '/' {
            slashes += 1;
            if slashes == 2 {
                new_line = new_line[..new_line.len() - 2].to_string();
            }
        } else {
            slashes = 0;
        }
    }
    let line = new_line.clone();

    //remove spaces from end
    for character in line.chars().rev() {
        if character == ' '{
            new_line = new_line[..new_line.len() - 1].to_string();
        } else {
            break;
        }
    }

    return new_line;
}