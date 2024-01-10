//mod
mod object;
mod constant;
mod physics;

use constant::constant::{SCREEN_HEIGHT, SCREEN_WIDTH, UPDATE_TIME_STEP, BALL_RADIUS, CHOC_RESTITUTION};
use object::object_mod::NewVelocity;
use physics::physics::{rk4_step, are_colliding, calculate_new_velocities};
//use
use raylib::prelude::Vector2;
use raylib::prelude::*;
use std::time::{Duration, Instant};

fn main() {
    //VEC of Ball

    let mut balls: Vec<object::object_mod::Ball> = Vec::new();
    balls.push(object::object_mod::Ball {
        position: Vector2 {
            x: (100) as f32,
            y: ((SCREEN_HEIGHT / 2)) as f32,
        },
        velocity: Vector2 { x: 400.0, y: 0.0 },
        acceleration: Vector2 { x: 0.0, y: 0.0 },
        mass: 10.0,
    });

    balls.push(object::object_mod::Ball {
        position: Vector2 {
            x: (500) as f32,
            y: ((SCREEN_HEIGHT / 2) + 0) as f32,
        },
        velocity: Vector2 { x: 0.0, y: 0.0 },
        acceleration: Vector2 { x: 0.0, y: 0.0 },
        mass: 1.0,
    });


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
            let mut new_velocities: Vec<NewVelocity> = balls
                .iter()
                .map(|ball| NewVelocity {
                    vx: ball.velocity.x,
                    vy: ball.velocity.y,
                })
                .collect();

            // Collision detection and velocity calculation
            for i in 0..balls.len() {
                for j in i + 1..balls.len() {
                    if are_colliding(&balls[i], &balls[j]) {
                        let (new_vel_i, new_vel_j) = calculate_new_velocities(&balls[i], &balls[j]);
                        new_velocities[i] = new_vel_i;
                        new_velocities[j] = new_vel_j;
                    }
                }
            }

            // Apply the new velocities
            for (ball, new_velocity) in balls.iter_mut().zip(new_velocities.iter()) {
                ball.velocity.x = new_velocity.vx;
                ball.velocity.y = new_velocity.vy;
            }
            // Update game logic here
            for i in 0..balls.len() {
                rk4_step(&mut balls[i], UPDATE_TIME_STEP);

                if balls[i].position.x <= 0.0 + BALL_RADIUS as f32 + 0.01 {
                    balls[i].velocity.x *= -1.0;
                    balls[i].position.x = 0.01 + BALL_RADIUS as f32;
                }
                if balls[i].position.x >= (SCREEN_WIDTH - BALL_RADIUS) as f32 - 0.01 {
                    balls[i].velocity.x *= -1.0;
                    balls[i].position.x = SCREEN_WIDTH as f32 - 0.01 - BALL_RADIUS as f32;
                }
                if balls[i].position.y <= 0.0 + BALL_RADIUS as f32 + 0.01 {
                    balls[i].velocity.y *= -1.0;
                    balls[i].position.y = 0.01 + BALL_RADIUS as f32;
                }
                if balls[i].position.y >= (SCREEN_HEIGHT - BALL_RADIUS) as f32 - 0.01 {
                    balls[i].velocity.y *= -1.0;
                    balls[i].position.y = SCREEN_HEIGHT as f32 - 0.01 - BALL_RADIUS as f32;
                }
            }

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

        d.draw_fps(d.get_screen_width() - 100, 10);
    }
}