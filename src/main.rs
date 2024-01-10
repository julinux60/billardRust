//mod
mod Ball;

//use
use raylib::prelude::Vector2;
use raylib::prelude::*;
use std::time::{Duration, Instant};

//type

struct NewVelocity {
    vx: f32,
    vy: f32,
}

const BALL_RADIUS: i32 = 10;

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 800;

const UPDATES_PER_SECOND: u64 = 1600;
const UPDATE_TIME_STEP: f32 = 1.0 / UPDATES_PER_SECOND as f32;

const CHOC_RESTITUTION:f32 = 1.0;

fn main() {
    //VEC of Ball

    let mut balls: Vec<Ball::object_mod::Ball> = Vec::new();
    balls.push(Ball::object_mod::Ball {
        position: Vector2 {
            x: (100) as f32,
            y: ((SCREEN_HEIGHT / 2)) as f32,
        },
        velocity: Vector2 { x: 400.0, y: 0.0 },
        acceleration: Vector2 { x: 0.0, y: 0.0 },
        mass: 10.0,
    });

    balls.push(Ball::object_mod::Ball {
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
        .title("Runge-Kutta 4 Integration - Elastic choc example")
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
            "Runge-Kutta 4 Integration - Elastic choc example",
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

fn scale_vector2(vector: &Vector2, factor: f32) -> Vector2 {
    Vector2 {
        x: vector.x * factor,
        y: vector.y * factor,
    }
}

fn add_vector2(v1: &Vector2, v2: &Vector2) -> Vector2 {
    Vector2 {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
    }
}

fn rk4_step(ball: &mut Ball::object_mod::Ball, dt: f32) {
    let k1_vel = scale_vector2(&ball.acceleration, dt);
    let k1_pos = scale_vector2(&ball.velocity, dt);

    let mid_velocity1 = add_vector2(&ball.velocity, &scale_vector2(&k1_vel, 0.5));
    let k2_vel = scale_vector2(&ball.acceleration, dt);
    let k2_pos = scale_vector2(&mid_velocity1, dt);

    let mid_velocity2 = add_vector2(&ball.velocity, &scale_vector2(&k2_vel, 0.5));
    let k3_vel = scale_vector2(&ball.acceleration, dt);
    let k3_pos = scale_vector2(&mid_velocity2, dt);

    let end_velocity = add_vector2(&ball.velocity, &k3_vel);
    let k4_vel = scale_vector2(&ball.acceleration, dt);
    let k4_pos = scale_vector2(&end_velocity, dt);

    ball.position = add_vector2(
        &ball.position,
        &scale_vector2(
            &add_vector2(
                &k1_pos,
                &add_vector2(
                    &scale_vector2(&k2_pos, 2.0),
                    &add_vector2(
                        &scale_vector2(&k3_pos, 2.0),
                        &k4_pos,
                    ),
                ),
            ),
            1.0 / 6.0,
        ),
    );
    ball.velocity = add_vector2(
        &ball.velocity,
        &scale_vector2(
            &add_vector2(
                &k1_vel,
                &add_vector2(
                    &scale_vector2(&k2_vel, 2.0),
                    &add_vector2(
                        &scale_vector2(&k3_vel, 2.0),
                        &k4_vel,
                    ),
                ),
            ),
            1.0 / 6.0,
        ),
    );
}


fn are_colliding(ball1: &Ball::object_mod::Ball, ball2: &Ball::object_mod::Ball) -> bool {
    let dx = ball1.position.x - ball2.position.x;
    let dy = ball1.position.y - ball2.position.y;
    let distance = (dx * dx + dy * dy).sqrt();

    distance < (2.0 * BALL_RADIUS as f32)
}


fn calculate_new_velocities(ball1: &Ball::object_mod::Ball, ball2: &Ball::object_mod::Ball) -> (NewVelocity, NewVelocity) {
    let dx = ball1.position.x - ball2.position.x;
    let dy = ball1.position.y - ball2.position.y;
    let distance = (dx * dx + dy * dy).sqrt();

    // Normalized collision vector
    let nx = dx / distance;
    let ny = dy / distance;

    // Relative velocity
    let vx = ball1.velocity.x - ball2.velocity.x;
    let vy = ball1.velocity.y - ball2.velocity.y;

    // Velocity along the normal
    let dot = vx * nx + vy * ny;

    // Don't resolve if velocities are separating
    if dot > 0.0 {
        return (
            NewVelocity { vx: ball1.velocity.x, vy: ball1.velocity.y },
            NewVelocity { vx: ball2.velocity.x, vy: ball2.velocity.y },
        );
    }

    // Calculate impulse scalar
    let impulse = ((1.0 + CHOC_RESTITUTION) * dot) / (ball1.mass + ball2.mass);

    // Calculate new velocities
    let new_vx_ball1 = ball1.velocity.x - impulse * ball2.mass * nx;
    let new_vy_ball1 = ball1.velocity.y - impulse * ball2.mass * ny;
    let new_vx_ball2 = ball2.velocity.x + impulse * ball1.mass * nx;
    let new_vy_ball2 = ball2.velocity.y + impulse * ball1.mass * ny;

    (NewVelocity { vx: new_vx_ball1, vy: new_vy_ball1 }, NewVelocity { vx: new_vx_ball2, vy: new_vy_ball2 })
}
