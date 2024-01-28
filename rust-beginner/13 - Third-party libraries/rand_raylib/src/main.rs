use raylib::prelude::*;
use rand::Rng;

const SCREEN_WIDTH: i32 = 1600;
const SCREEN_HEIGHT: i32 = 900;
const CELL_SIZE: i32 = 10;
const GRID_WIDTH: i32 = SCREEN_WIDTH / CELL_SIZE;
const GRID_HEIGHT: i32 = SCREEN_HEIGHT / CELL_SIZE;

fn gol_step(cells: &mut Vec<Vec<i32>>) {
    let mut new_cells = vec![vec![0; GRID_HEIGHT as usize]; GRID_WIDTH as usize];
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let mut neighbors = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || nx >= GRID_WIDTH || ny < 0 || ny >= GRID_HEIGHT {
                        continue;
                    }
                    // cells[nx][ny] is either 0 (dead) or 1 (alive)
                    neighbors += cells[nx as usize][ny as usize];
                }
            }
            if cells[x as usize][y as usize] == 1 {
                if neighbors == 2 || neighbors == 3 {
                    new_cells[x as usize][y as usize] = 1;
                }
            } else {
                if neighbors == 3 {
                    new_cells[x as usize][y as usize] = 1;
                }
            }
        }
    }
    *cells = new_cells;
}



fn place_glider(cells: &mut Vec<Vec<i32>>) {
    // a glider is a pattern that moves diagonally
    // 0 1 0
    // 0 0 1
    // 1 1 1
    cells[1][0] = 1;
    cells[2][1] = 1;
    cells[0][2] = 1;
    cells[1][2] = 1;
    cells[2][2] = 1;
}

fn randomize_cells(cells: &mut Vec<Vec<i32>>) {
    let mut rng = rand::thread_rng();
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            cells[x as usize][y as usize] = rng.gen_range(0..2);
        }
    }
}
fn draw_cells(d: &mut RaylibDrawHandle, cells: &Vec<Vec<i32>>) {
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            if cells[x as usize][y as usize] == 1 {
                d.draw_rectangle(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE, Color::BLACK);
            }
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();
    rl.set_target_fps(60);
    let mut paused = false;
    let mut cells = vec![vec![0; GRID_HEIGHT as usize]; GRID_WIDTH as usize];
    while !rl.window_should_close() {
        rl.set_window_title(&thread, format!("FPS: {}", rl.get_fps()).as_str());
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
        } else if d.is_key_pressed(KeyboardKey::KEY_R) {
            randomize_cells(&mut cells);
        } else if d.is_key_pressed(KeyboardKey::KEY_G) {
            place_glider(&mut cells);
        }
        if !paused {
            gol_step(&mut cells);
        }
        draw_cells(&mut d, &cells);
    }
}
