use std::{thread::sleep, time::Duration};

// Basic struct that represents a ball
struct Ball {
    x: i32,
    y: i32,
    x_speed: i32,
    y_speed: i32,
}

impl Ball {
    fn check_bounce(&mut self, width: i32, height: i32) {
        // Check if we're at the edge of the screen
        // If so, reverse direction
        // Note: this is a very simple bounce, and
        //       breaks when the ball is faster than
        //       one unit per frame
        //          - Exercise for the reader: fix this :)
        if self.x == 0 || self.x == width - 1 {
            self.x_speed = -self.x_speed;
        }
        if self.y == 0 || self.y == height - 1 {
            self.y_speed = -self.y_speed;
        }
    }
    fn move_ball(&mut self) {
        // Update the ball's position
        self.x += self.x_speed;
        self.y += self.y_speed;
    }
}

pub fn main() {
    // Clear screen so we can freely use the cursor in print_window()
    print!("{}[2J", 27 as char);
    let width: i32 = 60;
    let height: i32 = 20;
    let ball = Ball {
        x: width / 2,
        y: height / 2,
        x_speed: 1,
        y_speed: 1,
    };
    let ball1 = Ball {
        x: width / 3,
        y: height / 3,
        x_speed: 1,
        y_speed: 1,
    };
    let ball2 = Ball {
        x: width / 4,
        y: height / 4,
        x_speed: 1,
        y_speed: 1,
    };
    let mut balls = vec![ball, ball1, ball2];
    loop {
        print_window(&balls, width, height);
        for ball in &mut balls {
            ball.check_bounce(width, height);
            ball.move_ball();
        }
        // Pause our application to simulate 60 FPS
        sleep(Duration::from_millis(1000 / 60));
    }
}

fn print_window(balls: &Vec<Ball>, width: i32, height: i32) {
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
            let mut found = false;
            for ball in balls {
                if x == ball.x && y == ball.y {
                    // Found a ball, print a big O
                    output.push('O');
                    found = true;
                }
            }
            if !found {
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
