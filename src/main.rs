/*
By: <Your Name Here>
Date: 2025-05-09
Program Details: <Program Description Here>
*/

mod modules;

use crate::modules::animated_image::AnimatedImage;
use crate::modules::collision::check_collision;
use crate::modules::label::Label;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

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
        50.0,  // width
        100.0, // height
        200.0, // x position
        60.0,  // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

    let img2 = StillImage::new(
        "assets/background.png",
        screen_width(),  // width
        screen_height(), // height
        500.0,           // x position
        60.0,            // y position
        true,            // Enable stretching
        1.0,             // Normal zoom (100%)
    )
    .await;

    // Create multiple GIF animations at different positions

    let mut gif2 = AnimatedImage::from_gif("assets/p.gif", 300.0, 300.0, 100.0, 100.0, true).await;

    let mut gif3 = AnimatedImage::from_gif("assets/test.gif", 500.0, 400.0, 100.0, 100.0, true).await;

    let mut gif4 = AnimatedImage::from_gif("assets/tt.gif", 200.0, 500.0, 100.0, 100.0, true).await;

    let lbl_out = Label::new("Hello\nWorld", 50.0, 100.0, 30);

    // Movement speed for the image
    const MOVE_SPEED: f32 = 200.0;
    gif2.set_angle(93.0);
    loop {
        clear_background(RED);

        // Handle keyboard input
        let mut move_dir = vec2(0.0, 0.0);
        if is_key_down(KeyCode::Right) {
            move_dir.x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            move_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            move_dir.y += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            move_dir.y -= 1.0;
        }

        // Normalize for consistent speed in all directions
        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }

        // Calculate movement amount
        let movement = move_dir * MOVE_SPEED * get_frame_time();

        // Store original position
        let old_pos = img.pos();
        // Try X movement first
        if movement.x != 0.0 {
            img.set_x(img.get_x() + movement.x);
            if check_collision(&img, &img2, 1) || check_collision(&img, &gif2, 1) {
                img.set_x(old_pos.x);
            }
        }
        // Then Y movement
        if movement.y != 0.0 {
            img.set_y(img.get_y() + movement.y);
            if check_collision(&img, &img2, 1) || check_collision(&img, &gif2, 1) {
                img.set_y(old_pos.y);
            }
        }

        // Draw all GIF animations
        lbl_out.draw();
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
