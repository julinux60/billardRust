pub mod physics {

    use crate::{constant, object::object_mod::NewVelocity};
    use crate::constant::constant::{BALL_RADIUS, CHOC_RESTITUTION};
    use crate::object;

    use raylib::prelude::Vector2;
    use raylib::prelude::*;
    use std::time::{Duration, Instant};

    pub fn scale_vector2(vector: &Vector2, factor: f32) -> Vector2 {
        Vector2 {
            x: vector.x * factor,
            y: vector.y * factor,
        }
    }

    pub fn add_vector2(v1: &Vector2, v2: &Vector2) -> Vector2 {
        Vector2 {
            x: v1.x + v2.x,
            y: v1.y + v2.y,
        }
    }

    pub fn rk4_step(ball: &mut object::object_mod::Ball, dt: f32) {
        let k1_vel = scale_vector2(&ball.acceleration, dt);
        let k1_pos = scale_vector2(&ball.velocity, dt);

        let mid_velocity = add_vector2(&ball.velocity, &scale_vector2(&k1_vel, 0.5));
        let k2_vel = scale_vector2(&ball.acceleration, dt);
        let k2_pos = scale_vector2(&mid_velocity, dt);

        let mid_velocity = add_vector2(&ball.velocity, &scale_vector2(&k2_vel, 0.5));
        let k3_vel = scale_vector2(&ball.acceleration, dt);
        let k3_pos = scale_vector2(&mid_velocity, dt);

        let end_velocity = add_vector2(&ball.velocity, &k3_vel);
        let k4_vel = scale_vector2(&ball.acceleration, dt);
        let k4_pos = scale_vector2(&end_velocity, dt);

        ball.position = add_vector2(
            &ball.position,
            &scale_vector2(
                &add_vector2(
                    &k1_pos,
                    &scale_vector2(&add_vector2(&k2_pos, &scale_vector2(&k3_pos, 2.0)), 2.0),
                ),
                1.0 / 6.0,
            ),
        );
        ball.velocity = add_vector2(
            &ball.velocity,
            &scale_vector2(
                &add_vector2(
                    &k1_vel,
                    &scale_vector2(&add_vector2(&k2_vel, &scale_vector2(&k3_vel, 2.0)), 2.0),
                ),
                1.0 / 6.0,
            ),
        );
    }

    pub fn are_colliding(ball1: &object::object_mod::Ball, ball2: &object::object_mod::Ball) -> bool {
        let dx = ball1.position.x - ball2.position.x;
        let dy = ball1.position.y - ball2.position.y;
        let distance = (dx * dx + dy * dy).sqrt();

        distance < (2.0 * BALL_RADIUS as f32)
    }

    pub fn calculate_new_velocities(
        ball1: &object::object_mod::Ball,
        ball2: &object::object_mod::Ball,
    ) -> (NewVelocity, NewVelocity) {
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
                NewVelocity {
                    vx: ball1.velocity.x,
                    vy: ball1.velocity.y,
                },
                NewVelocity {
                    vx: ball2.velocity.x,
                    vy: ball2.velocity.y,
                },
            );
        }

        // Calculate impulse scalar
        let impulse = ((1.0 + CHOC_RESTITUTION) * dot) / (ball1.mass + ball2.mass);

        // Calculate new velocities
        let new_vx_ball1 = ball1.velocity.x - impulse * ball2.mass * nx;
        let new_vy_ball1 = ball1.velocity.y - impulse * ball2.mass * ny;
        let new_vx_ball2 = ball2.velocity.x + impulse * ball1.mass * nx;
        let new_vy_ball2 = ball2.velocity.y + impulse * ball1.mass * ny;

        (
            NewVelocity {
                vx: new_vx_ball1,
                vy: new_vy_ball1,
            },
            NewVelocity {
                vx: new_vx_ball2,
                vy: new_vy_ball2,
            },
        )
    }
}
