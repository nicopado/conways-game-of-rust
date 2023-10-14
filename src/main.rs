use std::io;

mod grid;
mod terminal;

const DEFAULT_WIDTH:usize = 30;
const DEFAULT_HEIGHT:usize = 20;
const DEFAULT_TIME_INTERVAL:usize = 100;

fn main() {
    println!("Conway's Game of Life in Rust");

    let width = get_user_input("Enter the width of the grid: ");
    let height = get_user_input("Enter the height of the grid: ");
    let interval = get_user_input("Enter the time interval (ms): ");

    let width = width.unwrap_or(DEFAULT_WIDTH);
    let height = height.unwrap_or(DEFAULT_HEIGHT);
    let interval = interval.unwrap_or(DEFAULT_TIME_INTERVAL);

    let mut grid = grid::initialize_grid(width, height);

    loop {
        terminal::clear_terminal();

        grid::print_grid(&grid);

        grid::update_grid(&mut grid);

        std::thread::sleep(std::time::Duration::from_millis(interval.try_into().unwrap()));
    }
}

fn get_user_input(prompt: &str) -> Option<usize> {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().ok()
}