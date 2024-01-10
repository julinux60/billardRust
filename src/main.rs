//mod
mod constant;
mod object;
mod physics;

use constant::constant::{
    BALL_RADIUS, CHOC_RESTITUTION, SCREEN_HEIGHT, SCREEN_WIDTH, UPDATE_TIME_STEP,
};
use object::object_mod::{create_cube_mesh, NewVelocity};
use physics::physics::{are_colliding, calculate_new_velocities, rk4_step, update_physics};
//use
use raylib::prelude::Vector2;
use raylib::prelude::*;
use std::time::{Duration, Instant};

fn main() {
    let mut cube1 = create_cube_mesh(
        200,
        200,
        3,
        3,
        1.0,
        Vector2 { x: 100.0, y: 0.0 },
        Vector2 { x: 100.0, y: 100.0 },
        2,
    );

    //VEC of Ball

    let mut balls: Vec<object::object_mod::Ball> = Vec::new();

    let mut last_update_time = Instant::now();
    let mut lag = 0.0;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Runge-Kutta 4 Integration - Hookes law example")
        .build();

    while !rl.window_should_close() {
        let current_time = Instant::now();
        let elapsed = current_time.duration_since(last_update_time);
        last_update_time = current_time;
        lag += elapsed.as_secs_f32();

        let mut d = rl.begin_drawing(&thread);

        while lag >= UPDATE_TIME_STEP {

            if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
                cube1.balls[0].acceleration += Vector2{x: 2000.0, y: 0.0};
            }

            update_physics(&mut cube1, UPDATE_TIME_STEP);

            lag -= UPDATE_TIME_STEP;
        }

        d.clear_background(Color::BLACK);
        d.draw_text(
            "Runge-Kutta 4 Integration - Hookes law example",
            12,
            12,
            20,
            Color::GRAY,
        );

        for i in 0..balls.len() {
            d.draw_circle(
                balls[i].position.x as i32,
                balls[i].position.y as i32,
                BALL_RADIUS as f32,
                Color::WHITE,
            );
        }

        for spring in &cube1.springs {
            let ball1 = &cube1.balls[spring.ball1_index];
            let ball2 = &cube1.balls[spring.ball2_index];

            // Calculate current length and compare with rest length
            let current_length = (ball2.position - ball1.position).length();
            let delta_length = current_length - spring.rest_length;

            // Determine color based on the state of the spring
            let color = if delta_length < 0.0 {
                // Compressed: More red for more compression
                Color::new(
                    255,
                    (255.0 * (1.0 + delta_length / spring.rest_length)) as u8,
                    0,
                    255,
                )
            } else if delta_length > 0.0 {
                // Stretched: More blue for more stretching
                Color::new(
                    0,
                    (255.0 * (1.0 - delta_length / spring.rest_length)) as u8,
                    255,
                    255,
                )
            } else {
                // At rest: Green
                Color::new(0, 255, 0, 255)
            };

            // Draw the spring as a line
            d.draw_line(
                ball1.position.x as i32,
                ball1.position.y as i32,
                ball2.position.x as i32,
                ball2.position.y as i32,
                color,
            );
        }

        for i in 0..cube1.balls.len() {
            d.draw_circle(
                cube1.balls[i].position.x as i32,
                cube1.balls[i].position.y as i32,
                BALL_RADIUS as f32,
                Color::WHITE,
            );
        }

        d.draw_fps(d.get_screen_width() - 100, 10);
    }
}
