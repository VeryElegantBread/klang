pub fn error(line: String, line_num: usize, positions: Vec<u32>, error_message: String, help: String) {
    println!("\x1b[0;31m");
    println!("ERROR ON LINE {}: {}", line_num + 1, error_message);
    println!("|");
    println!("| {}", line);

    let mut spaces = "".to_string();
    for _ in 0..positions[0] {
        spaces = spaces + " ";
    }

    let mut arrows = "".to_string();
    for _ in positions[0]..positions[1] {
        arrows = arrows + "^";
    }
    println!("| {}{}", spaces, arrows);
    if help != "".to_string() {
        println!("help: {}", help)
    }
    println!("\x1b[0m");
    std::process::exit(1)
}