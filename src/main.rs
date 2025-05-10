/*
By: <Your Name Here>
Date: 2025-05-09
Program Details: <Program Description Here>
*/

mod modules;

use macroquad::prelude::*;
use crate::modules::collision::check_collision;
use crate::modules::still_image::StillImage;
use crate::modules::label::Label;
use crate::modules::animated_image::AnimatedImage;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "hit_check".to_owned(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut img = StillImage::new(
        "assets/king.png",
        100.0,  // width
        200.0,  // height
        200.0,  // x position
        60.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let img2 = StillImage::new(
        "assets/hit_hover.png",
        100.0,  // width
        200.0,  // height
        500.0,  // x position
        60.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    
    // Create multiple GIF animations at different positions
  
    let mut gif2 = AnimatedImage::from_gif(
        "assets/p.gif", 
        300.0, 300.0,          
        100.0, 100.0,          
        true                   
    ).await;
    
    let mut gif3 = AnimatedImage::from_gif(
        "assets/test.gif", 
        500.0, 400.0,          
        100.0, 100.0,          
        true                   
    ).await;
    
    let mut gif4 = AnimatedImage::from_gif(
        "assets/tt.gif", 
        200.0, 500.0,          
        100.0, 100.0,          
        true                   
    ).await;

    let mut lbl_out = Label::new("Hello\nWorld", 50.0, 100.0, 30);
    
    // Movement speed for the image
    const MOVE_SPEED: f32 = 1.0;
    
    loop {
        clear_background(RED);
        
        // Handle keyboard input
        let mut x_pos = img.pos().x;
        let mut y_pos = img.pos().y;
        
        if is_key_down(KeyCode::Right) {
            x_pos += MOVE_SPEED;
        }
        if is_key_down(KeyCode::Left) {
            x_pos -= MOVE_SPEED;
        }
        if is_key_down(KeyCode::Down) {
            y_pos += MOVE_SPEED;
        }
        if is_key_down(KeyCode::Up) {
            y_pos -= MOVE_SPEED;
        }
        
        // Update the image position
        img.set_position(vec2(x_pos, y_pos));
        
        lbl_out.draw();
        if check_collision(&img, &img2, 1) || check_collision(&img, &gif2, 1) {
            lbl_out.set_text("Collision Detected");
        } else {
            lbl_out.set_text("No Collision");
        }


        // Draw all GIF animations
       
        gif2.draw();
        gif3.draw();
        gif4.draw();
        
        img.draw();
        img2.draw();
        
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        next_frame().await;
    }
}
