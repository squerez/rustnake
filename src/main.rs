use raylib::prelude::*;
mod utils;
use std::time;


fn main() {
    // Hardcoded window number because of the pixel size
    // TODO make it dynamic
    let win_height: i32 = 515;
    let win_width: i32 = 480;
    let (mut rl, thread) = raylib::init()
        .size(win_width, win_height)
        .title("rustsnake")
        .build();

    // Comment to see performance
    // rl.set_target_fps(60);

    // Converting the whole area as if it was pixels
    let pixel_pad: utils::VECTOR2D = utils::VECTOR2D {x: 0, y: 35};
    let pixel_size: utils::VECTOR2D = utils::VECTOR2D {x: 20, y: 20};
    let game_area: utils::BBOX = utils::BBOX {xmin: 0,
                                              ymin: 0,
                                              xmax: 23,
                                              ymax: 23};
    
    // Create Snake
    let mut new_direction: utils::DIRECTION = utils::DIRECTION::Right;
    let mut snake: utils::SNAKE = utils::SNAKE::new();

    // Create Food
    let mut food: utils::VECTOR2D = utils::VECTOR2D{x:0,y:0};
    utils::gen_food(&mut food, &game_area);
    
    // Snake speed
    let snake_speed: time::Duration = time::Duration::new(0, 150_000_000);
    let mut tic: time::Instant = time::Instant::now();
    let mut toc: time::Instant;

    // Track score
    // Size of snake is according to score
    let mut score: i32 = 0;
    let mut score_test: String;

    // Object to get the real coordinates
    let mut real_coords: utils::VECTOR2D;

    while !rl.window_should_close() {
        // Start Drawing
        let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // Draw score
        score_test = format!("Score: {score}");
        d.draw_text(score_test.as_str(), 12, 12, 20, Color::BLACK);
        d.draw_rectangle(0, 30, win_width, 5, Color::RED);
        d.draw_fps(370,12);

        // Check for inputs
        new_direction = utils::get_input_dir(new_direction, snake.direction, &d);

        // Update logic according to the snake speed
        toc = time::Instant::now();
        if (toc-tic) > snake_speed{
            tic = time::Instant::now();
            // Check if it got food
            if snake.body[0].x == food.x && snake.body[0].y==food.y {
                utils::gen_food(&mut food, &game_area);
                score += 1;
            };
            snake.direction = new_direction;
            if !utils::update_snake(&mut snake, score, &game_area){break};
        }

        // Draw Food
        real_coords = utils::update_real_pos(&food, &pixel_pad, &pixel_size);
        d.draw_rectangle(real_coords.x, real_coords.y, pixel_size.x, pixel_size.y, Color::GREEN);

        // Draw Snake
        real_coords = utils::update_real_pos(&snake.body[0], &pixel_pad, &pixel_size);
        d.draw_rectangle(real_coords.x, real_coords.y, pixel_size.x, pixel_size.y, Color::VIOLET);
        for i in 1..(score+1) as usize{
            real_coords = utils::update_real_pos(&snake.body[i], &pixel_pad, &pixel_size);
            d.draw_rectangle(real_coords.x, real_coords.y, pixel_size.x, pixel_size.y, Color::BLUE);
        }
    }
}
