// Please shut up about unused functions, Rust
// We're learning here, not writing production code
#![allow(dead_code)]

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

fn main() {
    // The size of the window
    // Note: This is not the size of the terminal window
    //       but the size of the "play area" of the game
    let width: i32 = 120;
    let height: i32 = 40;
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
        for b in &mut balls {
            b.check_bounce(width, height);
            b.move_ball();
        }
        // Sleep for a bit to slow down the game
        std::thread::sleep(std::time::Duration::from_millis(1000 / 60)); // ~60 fps
    }
}


fn print_window(balls: &Vec<Ball>, width: i32, height: i32) {
    print!("{}[2J", 27 as char); // clear screen
    let mut output = String::new();
    for y in 0..height {
        for x in 0..width {
            // I'll leave this as an exercise for the reader
            // to make this more efficient :)
            let mut is_ball = false;
            for b in balls {
                if x == b.x && y == b.y {
                    output.push('O');
                    is_ball = true;
                    break;
                }
            }
            // If we didn't find a ball, print a space
            if !is_ball {
                output.push(' ');
            }
        }
        output.push('|');
        output.push('\n');
    }
    for _ in 0..width {
        output.push('-');
    }
    println!("{}", output);
}

fn main1() {
    let width: i32 = 60;
    let height: i32 = 40;
    let mut ball_x: i32 = width / 2;
    let mut ball_y: i32 = height / 2;
    let mut ball_x_speed: i32 = 1;
    let mut ball_y_speed: i32 = 1;
    loop {
        print_window1(ball_x, ball_y, width, height);
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
        std::thread::sleep(std::time::Duration::from_millis(1000 / 60)); // ~60 fps
    }
}

fn print_window1(ball_x: i32, ball_y: i32, width: i32, height: i32) {
    print!("{}[2J", 27 as char); // clear screen
    let mut output = String::new();
    for y in 0..height {
        for x in 0..width {
            if x == ball_x && y == ball_y {
                output.push('O');
            } else {
                output.push(' ');
            }
        }
        output.push('|');
        output.push('\n');
    }
    for _ in 0..width {
        output.push('-');
    }
    println!("{}", output);
}