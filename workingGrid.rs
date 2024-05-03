extern crate piston_window;
use piston_window::*;
use std::collections::HashMap;

// the only issue with this in it's current working state is that it cuts off the bottom of the grid
const GRID_SIZE: f64 = 40.0;
const WIDTH: i32 = 7;
const HEIGHT: i32 = 10;
const GRID_COLOR: [f32; 4] = [0.133, 0.133, 0.133, 1.0];

struct Game {
    // active squares are the squares that are currently moving
    active_squares: Vec<[f64; 4]>,
    // frozen squares are not one dimensional, and are stored in a hashmap
    frozen_squares: HashMap<(i32, i32), [f64; 4]>,
    // speed is a floating point, and is used to determine how fast the block is moving
    // the speed is increased with each level
    // we want to move in a smooth block by block motion, which will make it easier to stack the blocks
    speed: f64,
    // direction is a floating point number, and is used to determine which direction the block is moving
    // we switch direction by multiplying by -1
    direction: f64,
    // level is an integer
    // there are three levels, the first level has 3 blocks, the second level has 2 blocks, and the third level has 1 block
    // the speed is also increased with each level
    level: i32,
}

impl Game {
    fn new() -> Game {
        // the game begins at Level 1
        // This means the speed is 1, the block is moving right, and there are 3 blocks
        let mut game = Game {
            active_squares: vec![],
            frozen_squares: HashMap::new(),
            speed: 1.0,
            direction: 1.0,
            level: 1,
        };
        // spawn a that is 3 blocks wide, one blocks tall
        game.spawn_block(3); 
        game
    }

    // update is a function that will be called after each user input
    fn update(&mut self, keyboard: &Button) {
        println!("Updating game state...");
        if let Button::Keyboard(key) = keyboard {
            if let Key::Space = key {
                println!("Spacebar pressed, placing block...");
                self.place_block();
            }
        }

        if self.active_squares.is_empty() {
            println!("No active squares, spawning new block...");
            // tells the game which level it is on, and decreases the block size by 1
            let size = match self.level {
                // first three levels have 3 blocks, the next three levels have 2 blocks, and the remaining levels have 1 block
                1..=3 => 3,
                4..=6 => 2,
                _ => 1,
            };
            // the location needs to be at the bottom of the screen, not the top
            self.spawn_block(size);
        }

        println!("Moving block...");
        self.move_block(0.0); // For now, using a constant dt for debugging

        println!("Checking for collision...");
        self.check_collision();

        // need a way to increment the speed
        if self.active_squares.is_empty() && self.frozen_squares.is_empty() {
            println!("Increasing speed...");
            // increase the speed by a factor of 1.1
            self.speed *= 1.1;
        }
        println!("Game state update successful.");
    }

    fn spawn_block(&mut self, size: i32) {
        let x_position = WIDTH as f64 * GRID_SIZE / 2.0 - size as f64 * GRID_SIZE / 2.0;
        for index in 0..size {
            self.active_squares.push([
                GRID_SIZE * index as f64 + x_position,
                HEIGHT as f64 * GRID_SIZE - GRID_SIZE,
                GRID_SIZE,
                GRID_SIZE,
            ]);
        }
    }

    fn move_block(&mut self, dt: f64) {
        println!("Moving block...");
        // we want to block to move to the right, then when it hits the wall, move to the left, and so on
        for square in self.active_squares.iter_mut() {
            square[0] += self.speed * dt * self.direction * GRID_SIZE;
        }
        println!("Block moved.");
    }

    fn check_collision(&mut self) {
        println!("Checking for collision...");
        let mut collided = false;
        for square in &self.active_squares {
            if square[0] <= 0.0 || square[0] + GRID_SIZE >= WIDTH as f64 * GRID_SIZE {
                collided = true;
                break;
            }
        }
        if collided {
            // Change direction
            self.direction *= -1.0;
        }
        println!("Collision checked.");
    }

    fn place_block(&mut self) {
        if self.active_squares.is_empty() {
            let size = match self.level {
                1..=3 => 3,
                4..=6 => 2,
                _ => 1,
            };
            self.spawn_block(size);
            self.frozen_squares.extend(
                self.active_squares
                    .iter()
                    .map(|&pos| ((pos[0] / GRID_SIZE) as i32, (pos[1] / GRID_SIZE) as i32),)
                    .zip(self.active_squares.iter().copied()),
            );
            self.active_squares.clear();
            self.level += 1;
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Stacker Game",
        [(WIDTH as f64 * GRID_SIZE) as u32, (HEIGHT as f64 * GRID_SIZE) as u32],
    )
    // simple way to close the window
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new();

    while let Some(e) = window.next() {
        if let Some(args) = e.button_args() {
            game.update(&args.button);
        }

        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                clear([1.0; 4], g);
                for x in (0..WIDTH).map(|x| x as f64 * GRID_SIZE) {
                    line(
                        GRID_COLOR,
                        2.0,
                        [x, 0.0, x, HEIGHT as f64 * GRID_SIZE],
                        c.transform,
                        g,
                    );
                }
                for y in (0..HEIGHT).map(|y| y as f64 * GRID_SIZE) {
                    line(
                        GRID_COLOR,
                        2.0,
                        [0.0, y, WIDTH as f64 * GRID_SIZE, y],
                        c.transform,
                        g,
                    );
                }
                for square in &game.active_squares {
                    rectangle([1.0, 0.0, 0.0, 1.0], *square, c.transform, g);
                }
                for &val in game.frozen_squares.values() {
                    rectangle([0.0, 1.0, 0.0, 1.0], val, c.transform, g);
                }
            });
        }
    }
}