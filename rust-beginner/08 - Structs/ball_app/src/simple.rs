use std::{thread::sleep, time::Duration};

fn print_window(ball_x: i32, ball_y: i32, width: i32, height: i32) {
    // Move the cursor of the terminal a few lines up so we don't spam the terminal
    print!("{}[{}A", 27 as char, width);
    let mut output = String::new();
    // \u{2588} = █
    output.push_str(&"\u{2588}".repeat(width as usize + 4));
    output.push('\n');
    for y in 0..height {
        output.push('\u{2588}');
        output.push('\u{2588}');
        for x in 0..width {
            if x == ball_x && y == ball_y {
                // Found a ball, print a big O
                output.push('O');
            } else {
                // If we didn't find a ball, print a space
                output.push(' ');
            }
        }
        output.push('\u{2588}');
        output.push('\u{2588}');
        output.push('\n');
    }
    output.push_str(&"\u{2588}".repeat(width as usize + 4));
    println!("{}", output);
}

pub fn main() {
    // Clear screen so we can freely use the cursor in print_window()
    print!("{}[2J", 27 as char);
    let width = 60;
    let height = 20;
    let mut ball_x = width / 2;
    let mut ball_y = height / 2;
    let mut ball_x_speed = 1;
    let mut ball_y_speed = 1;
    loop {
        print_window(ball_x, ball_y, width, height);
        // Ball bounce
        if ball_x == 0 || ball_x == width - 1 {
            ball_x_speed = -ball_x_speed;
        }
        if ball_y == 0 || ball_y == height - 1 {
            ball_y_speed = -ball_y_speed;
        }
        // Ball move
        ball_x += ball_x_speed;
        ball_y += ball_y_speed;
        // Pause our application to simulate 60 FPS
        sleep(Duration::from_millis(1000 / 60));
    }
}