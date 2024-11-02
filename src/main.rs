use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Duration;

pub fn main() {
    let width = 1200;
    let height: i32 = 1000;

    let mut game_running = true;

    let mut pong_x: i32 = 500;
    let mut pong_y: i32 = rand::thread_rng().gen_range(50..width-50);
    let mut going_up = true;
    let mut going_right = true;
    let pong_x_speed = 5;
    let pong_y_speed = 5;

    let mut player_1_y: i32 = 500;
    let mut player_2_y: i32 = 500;
    let player_speed: i32 = 6;
    let mut player_1_score = 0;
    let mut player_2_score = 0;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("pong-rust", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut rng = rand::thread_rng();
    'running: loop {
        for event in event_pump.poll_iter() { // Input to exit the game
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Input
        if game_running {
            let movement = handle_movement(&event_pump);
            // Ensure pong_y stays within the screen bounds
            if pong_y <= 0 {
                pong_y = 0; // Reset to 0 if it goes out of bounds
                going_up = false;
            }
            if pong_y >= height {
                pong_y = height; // Reset to height if it goes out of bounds
                going_up = true;
            }
            // Player 1 Movement
            if movement[0] == -1 {
                if player_1_y < height - 100 { // Make sure we don't exceed the lower bound
                    player_1_y += player_speed;
                }
            } else if movement[0] == 1 {
                if player_1_y > 100 { // Prevent player_1_y from going below 0
                    player_1_y -= player_speed;
                }
            }
            
            // Player 2 Movement
            if movement[1] == -1 {
                if player_2_y < height - 100 { // Make sure we don't exceed the lower bound
                    player_2_y += player_speed;
                }
            } else if movement[1] == 1 {
                if player_2_y > 100 { // Prevent player_2_y from going below 0
                    player_2_y -= player_speed;
                }
            }

            if going_right {
                pong_x += pong_x_speed; // Make the pong go right
            }
            if !going_right {
                pong_x -= pong_x_speed; // Make the pong go left
            }

            if going_up {
                pong_y-=pong_y_speed;
            }
            if !going_up {
                pong_y+=pong_y_speed;
            }
        }

        // Ensure pong_y stays within the screen bounds
        if pong_y <= 0 {
            pong_y = 0; // Reset to 0 if it goes out of bounds
            going_up = false;
        }
        if pong_y >= height {
            pong_y = height; // Reset to height if it goes out of bounds
            going_up = true;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0)); // Set the background colour to black
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255)); // Set the draw colour to white for the next graphics

        canvas.draw_rect(Rect::new(pong_x,pong_y,5,5)); // Draw the pong

        canvas.draw_line(Point::new(1,player_1_y-100), Point::new(1,player_1_y+100)); // Draw Player 1
        canvas.draw_line(Point::new(width-1,player_2_y-100), Point::new(width-1,player_2_y+100)); // Draw Player 2

        // Draw boundry
        canvas.draw_line(Point::new(0,height), Point::new(width,height));
        canvas.draw_line(Point::new(0,-height), Point::new(width,-height));
        
        canvas.draw_line(Point::new(-1,height), Point::new(-1,0));
        canvas.draw_line(Point::new(width,height), Point::new(width,0));

        if pong_x >= width-1{
            if pong_y > player_2_y-100 && pong_y < player_2_y+100 {
                going_right = false;
            }else{
                println!("PLAYER 1 WINS!");
                player_1_score+=1;
                println!("{player_1_score} - {player_2_score}");
                pong_x = 500;
                pong_y = rand::thread_rng().gen_range(50..width-50);
                player_1_y = 500;
                player_2_y = 500;
                game_running = false;
                ::std::thread::sleep(Duration::new(1,0)); // sloppy FPS limit
                game_running = true;
            }
        }
        if pong_x <= 1 {
            if pong_y > player_1_y-100 && pong_y < player_1_y+100 {
                going_right = true;
            }else{
                println!("PLAYER 2 WINS!");
                player_2_score+=1;
                println!("{player_1_score} - {player_2_score}");
                pong_x = 500;
                pong_y = rand::thread_rng().gen_range(50..width-50);
                player_1_y = 500;
                player_2_y = 500;
                game_running = false;
                ::std::thread::sleep(Duration::new(1,0)); // sloppy FPS limit
                game_running = true;
            }
        }
        if pong_y <= 0 {
            going_up = false;
        }
        if pong_y >= height {
            going_up = true;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // sloppy FPS limit
    }
}

pub fn handle_movement(input: &sdl2::EventPump) -> Vec<i32>{
    let mut v: Vec<i32> = vec![0,0];
    if input.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::W){
        v[0] = 1;
    }else if input.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::S){
        v[0] = -1;
    }
    if input.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::Up){
        v[1] = 1;
    }else if input.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::Down){
        v[1] = -1;
    }
    v
}