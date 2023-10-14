pub fn initialize_grid(width: usize, height: usize) -> Vec<Vec<bool>> {
    // Example: glider
    let mut grid = vec![vec![false; width]; height];
    grid[1][2] = true;
    grid[2][3] = true;
    grid[3][1] = true;
    grid[3][2] = true;
    grid[3][3] = true;
    grid
}

pub fn print_grid(grid: &[Vec<bool>]) {
    for row in grid {
        for &cell in row {
            print!("{}", if cell { "#" } else { " " });
        }
        println!();
    }
}

pub fn update_grid(grid: &mut Vec<Vec<bool>>) {
    let mut new_grid = grid.clone();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let neighbors = count_neighbors(&grid, x, y);

            new_grid[y][x] = match (grid[y][x], neighbors) {
                (true, 2) | (true, 3) => true,   // Cell survives
                (false, 3) => true,              // Cell is born
                _ => false,                      // Cell dies
            };
        }
    }

    *grid = new_grid;
}

fn count_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> u8 {
    let mut count = 0;

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];

    for &(dx, dy) in &directions {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        if new_x >= 0 && new_x < grid[0].len() as i32 && new_y >= 0 && new_y < grid.len() as i32 {
            count += grid[new_y as usize][new_x as usize] as u8;
        }
    }

    count
}