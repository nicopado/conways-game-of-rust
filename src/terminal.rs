pub fn clear_terminal() {
    print!("{}[2J", 27 as char);
    print!("{}[0;0H", 27 as char);
}