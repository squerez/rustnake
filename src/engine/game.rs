use raylib::prelude::*;

// Import linked list to store snake and to calculate throttle speed
use std::collections::LinkedList;
use std::time;

// Import random to spawn food
use rand::Rng;

// Import configs
use super::configs::*;

// Define a game
pub struct Game {
    pub snake: LinkedList<SnakeSegment>,
    pub direction: Direction,
    pub food: SnakeSegment,
    pub game_over: bool,
    pub score: i32,
    pub start_instant: time::Instant,
    pub update_speed: time::Duration
}


impl Game {
    // Implements a game struct with well defined functionality.
    // Each method contains a different part of the logic of the snake game
    pub fn new() -> Self {
        // Creates a new instance of a snake in the form of a `LinkedList`.
        // The elements of the snake are provided as a vector with x and y coordinates.
        // The snake starts with only one element
        let snake = LinkedList::from_iter(vec![SnakeSegment { x: 3, y: 5 },]);

        // Creates a new instance of a food in the form of a vector with x and y coordinates.
        // TODO: Randomize food 
        let food = SnakeSegment { x: 10, y: 5 };

        // A new game is now returned where the properties are instantiated already
        Game {
            snake,
            direction: Direction::Right,
            food,
            game_over: false,
            score: 0,
            start_instant: time::Instant::now(),
            update_speed: time::Duration::new(0, 75_000_000)
        }
    }

    pub fn update(&mut self) {
        // Check if the game is over and handle the termination logic to main.rs
        if self.game_over { return; }

        // A new head is created by cloning the first segment of the snake
        let mut snake_head = *self.snake.front().expect("Snake has no segments");

        // Updates the position of the snake head based on the current direction
        // Whenever the head goes out of bounds, it wraps around to the opposite side of the screen
        match self.direction {
            Direction::Up => {
                snake_head.y -= 1;

                // In this case it goes to the top of screen
                if snake_head.y < 0 {
                    snake_head.y = SCREEN_HEIGHT / GRID_SIZE - 1;
                }
            }

            Direction::Down => {
                snake_head.y += 1;

                // In this case it goes to the bottom of screen
                if snake_head.y >= SCREEN_HEIGHT / GRID_SIZE {
                    snake_head.y = 0;
                }
            }

            Direction::Left => {
                snake_head.x -= 1;

                // In this case it goes to the right of screen
                if snake_head.x < 0 {
                    snake_head.x = SCREEN_WIDTH / GRID_SIZE - 1;
                }
            }

            Direction::Right => {
                snake_head.x += 1;

                // In this case it goes to the left of screen
                if snake_head.x >= SCREEN_WIDTH / GRID_SIZE {
                    snake_head.x = 0;
                }
            }
        }

        // Check if any segment of the snake's body except the head has the same coordinates as
        // the head
        // If it does, it means the snake has collided with itself and the game is over
        let snake_has_collided: bool = 
            self.snake.iter().skip(1)
            .any(|segment| segment.x == snake_head.x && segment.y == snake_head.y)
        ;
        if snake_has_collided {
            self.game_over = true;
            return;
        }

        // In this section we throttle the update to make snake speed bearable for the user
        let end_instant: time::Instant = time::Instant::now();
        if (end_instant - self.start_instant) > self.update_speed {
            // Recalculate time 
            self.start_instant = time::Instant::now();
            
            // Check if the food has been eaten
            let food_has_been_eaten: bool = 
                (snake_head.x == self.food.x) 
                && 
                (snake_head.y == self.food.y);

            if food_has_been_eaten {
                self.score += 1;
                self.spawn_food();
            } else {
                // Else, the last element is removed
                self.snake.pop_back();
            }

            // And the updated head is pushed to the front to simulate movement
            self.snake.push_front(snake_head);
        }
    }

    fn spawn_food(&mut self) {
        // Generates the food in a new position and assigns it to the self property.
        let mut rng = rand::thread_rng();

        loop {
            let x = rng.gen_range(0..(SCREEN_WIDTH / GRID_SIZE ));
            let y = rng.gen_range(0..(SCREEN_HEIGHT / GRID_SIZE ));

            // It checks if the newly generated food is colliding with any segment of the snakes
            // body
            let is_colliding = self
                .snake
                .iter()
                .any(|segment| segment.x == x && segment.y == y);

            // If no collision is detected, then the new food is generated
            // Else the game will try to find a new position for the food in the next loop
            if !is_colliding {
                self.food = SnakeSegment { x, y };
                break;
            }
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle) {
        // Function responsible for rendering the objects of the snake in raylib
        // Starts by clearing pre-existing objects
        d.clear_background(Color::WHITE);

        // First render the snake body by iterating through each segment 
        // and using enumerate to get the index
        // The index is important to color the head of the snake differently from the body
        for (index, segment) in self.snake.iter().enumerate() {

            // Calculates the pixel coordinates on the grid
            let x = segment.x * GRID_SIZE;
            let y = segment.y * GRID_SIZE;

            // For the snake we make the it look smaller and add a border
            // in the same colour of the bg to make it look cooler
            // This draw will only draw a border around the body and head
            let body_size: i32 = (GRID_SIZE as f32 * 0.8) as i32;
            let border_size: i32 = (GRID_SIZE - body_size) / 2;

            d.draw_rectangle(
                x + border_size,
                y + border_size,
                body_size,
                body_size,
                Color::WHITE,
            );

            // We populate the snake head color differently from the body
            // To make it pop
            let color = if index == 0 {
                Color::BLACK 
            } else {
                Color::GRAY 
            };

            // Now we draw the whole snake with the different colors
            d.draw_rectangle(
                x + border_size + 1,
                y + border_size + 1,
                body_size - 2,
                body_size - 2,
                color,
            );
        }

        // We draw the food in a separate part to avoid the looping 
        let food_x = self.food.x * GRID_SIZE;
        let food_y = self.food.y * GRID_SIZE;
        d.draw_rectangle(
            food_x,
            food_y,
            GRID_SIZE,
            GRID_SIZE,
            Color::RED,
        );

        // Finally we draw the score somewhere
        let score_text = format!("Score: {}", self.score);
        d.draw_text(&score_text, 10, 10, 20, Color::BLACK);

        // If the game is over we also draw it somewhere in the screen
        if self.game_over {
            d.draw_text(
                "Game Over",
                SCREEN_WIDTH / 2 - 70,
                SCREEN_HEIGHT / 2 - 10,
                40,
                Color::BLACK,
            );
        }
    }
}
