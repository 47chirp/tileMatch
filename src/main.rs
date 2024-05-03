extern crate piston_window;
use piston_window::*;
use raylib::ffi::rand;
use std::collections::HashMap;
use rand::Rng;

// Constants
// grid size is the size of each square in the grid
const GRID_SIZE: f64 = 40.0;
// width and height are the number of squares in the grid
const WIDTH: i32 = 7;
const HEIGHT: i32 = 15;
// grid color is the color of the grid
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
    // without this, the blocks will just stay at the same level on the y-axis
    current_y: f64,
}

impl Game {
    fn new() -> Game {
        Game {
            // the game begins at Level 1
            active_squares: vec![],
            frozen_squares: HashMap::new(),
            speed: 1.5,
            // the block will move right
            direction: 1.0,
            // the game starts at level 1 (meaning 3 blocks, speed 1.5, and moving right)
            level: 1,
            current_y: HEIGHT as f64 * GRID_SIZE - GRID_SIZE,
        }
    }

    fn update(&mut self, dt: f64) {
        // if there are no active squares, spawn a new block
        // this coincides with the frozen squares, which are the squares that have already been placed
        if self.active_squares.is_empty() {
            // the size of the block depends on the level
            let size = match self.level {
                // when the level is 1, 2, or 3, the size is 3
                1..=3 => 3,
                // when the level is 4, 5, or 6, the size is 2
                4..=6 => 2,
                // when the level is 7 or higher, the size is 1
                _ => 1,
            };
            // spawn the block for the given size
            self.spawn_block(size);
        }

        // move the block and check for collision
        self.move_block(dt);
        self.check_collision();
    }

    // next, we must account for user inputs to place the block when the space key is pressed
    fn update_on_button_press(&mut self, button: &Button) {
        // we set the button to a keyboard key
        if let Button::Keyboard(key) = button {
            // if the key is the space key, we place the block
            if let Key::Space = key {
                self.place_block();
            }
        }
    }

    // we have come up with a system to place the block, now  we must implement the spawn_block function
    // this function will spawn a block of a given size
    fn spawn_block(&mut self, size: i32) {
        let mut rng = rand::thread_rng();
        // we set the x position to a random position on the x-axis
        // in doing so mechanics like spamming the space key to get the block to the top will be prevented
        let max_pot_x_position = WIDTH as f64 * GRID_SIZE - size as f64 * GRID_SIZE;
        let x_position = rng.gen_range(0.0..max_pot_x_position);
        
        // for the given size, we push the active squares to the active_squares vector
        for index in 0..size {
            self.active_squares.push([
                // x position is the random x position we plus the grid size times the index
                x_position + GRID_SIZE * index as f64,
                // y position is the current y position
                self.current_y,
                // the width of the square is the grid size, and the height of the square is the grid size
                GRID_SIZE,
                GRID_SIZE,
            ]);
        }
    }

    // function that moves the block
    fn move_block(&mut self, dt: f64) {
        for square in self.active_squares.iter_mut() {
            // we move the block by the speed and direction
            square[0] += self.speed * dt * self.direction * GRID_SIZE;
        }
    }

    // checking to see if the block has collided with the left or right side of the grid
    fn check_collision(&mut self) {
        // assume that the block has not collided
        let mut collided = false;
        // check if the block has collided with the left or right side of the grid
        for square in &self.active_squares {
            // if the block has collided with the left or right side of the grid, we set collided to true
            if square[0] <= 0.0 || square[0] + GRID_SIZE >= WIDTH as f64 * GRID_SIZE {
                collided = true;
                break;
            }
        }
        // if the block has collided, we must reverse the direction
        // creating a bouncing effect
        if collided {
            self.direction *= -1.0;
        }
    }

    // function to place the block
    fn place_block(&mut self) {
        // we add the active squares to the frozen squares when the block is placed
        self.frozen_squares.extend(
            // we map the active squares to the grid position and the square
            self.active_squares.iter().map(|&pos| ((pos[0] / GRID_SIZE) as i32, (pos[1] / GRID_SIZE) as i32)).zip(self.active_squares.iter().copied())
        );
        // we clear the active squares
        self.active_squares.clear();
        // we increase the level
        self.level += 1;
        // we decrease the current y position (to move up one square)
        self.current_y -= GRID_SIZE;
        // if the current y position is less than 0, we reset the current y position
        if self.current_y < 0.0 {
            self.current_y = HEIGHT as f64 * GRID_SIZE - GRID_SIZE;
        }

        // this is the logic for the block size (explained earlier)
        let new_block_size = match self.level {
            1..=3 => 3,
            4..=6 => 2,
            _ => 1,
        };
        // we spawn the block with the new block size when needed
        self.spawn_block(new_block_size);
        // we increase the speed after each placement
        self.increase_speed(); 
    }

    fn increase_speed(&mut self) {
        // Increase speed by 35% each time a block is placed
        self.speed *= 1.35;
    }
}

fn main() {
    // we create a window with the title "Stacker Game" and the size of the window
    let mut window: PistonWindow = WindowSettings::new("Stacker Game", [(WIDTH as f64 * GRID_SIZE) as u32, (HEIGHT as f64 * GRID_SIZE) as u32])
        // we set the exit on esc to true
        // this will allow the user to exit the game by pressing the esc key
        .exit_on_esc(true)
        .build()
        .unwrap();

    // we create a new game
    let mut game = Game::new();

    //game loop
    while let Some(e) = window.next() {
        // we check for button presses, specifically the space key
        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                game.update_on_button_press(&args.button);
            }
        }
        // we update the game when needed
        if let Some(args) = e.update_args() {
            game.update(args.dt);
        }
        // we render the game after updating
        if let Some(_) = e.render_args() {
            // we draw the game
            window.draw_2d(&e, |c, g, _| {
                // we clear the screen
                clear([1.0; 4], g);
                // then we draw the grid for both the respective x and y lines
                for x in (0..WIDTH).map(|x| x as f64 * GRID_SIZE) {
                    line(GRID_COLOR, 2.0, [x, 0.0, x, HEIGHT as f64 * GRID_SIZE], c.transform, g);
                }
                for y in (0..HEIGHT).map(|y| y as f64 * GRID_SIZE) {
                    line(GRID_COLOR, 2.0, [0.0, y, WIDTH as f64 * GRID_SIZE, y], c.transform, g);
                }
                // at this stage we have a working empty grid but need to add the logic for the active squares and the frozen squares
                // we loop through the active squares and the frozen squares and draw them
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