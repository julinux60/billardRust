pub mod object_mod {
    use raylib::prelude::Vector2;

    use crate::constant::constant::STIFFNESS_COEFF;
    #[derive(Clone)]
    pub struct Ball {
        pub position: Vector2,
        pub velocity: Vector2,
        pub acceleration: Vector2,
        pub mass: f32,
    }

    pub struct NewVelocity {
        pub vx: f32,
        pub vy: f32,
    }

    pub struct Spring {
        pub ball1_index: usize, // Index of the first ball in the balls vector
        pub ball2_index: usize, // Index of the second ball
        pub rest_length: f32,
        pub stiffness: f32,
    }

    pub struct JMesh {
        pub balls: Vec<Ball>,
        pub springs: Vec<Spring>,
    }

    pub fn create_cube_mesh(
        size_x: usize,
        size_y: usize,
        subdivision_x: usize,
        subdivision_y: usize,
        mass_balls: f32,
        initial_velocity: Vector2,
        cube_position: Vector2,
        spring_method: u8,
    ) -> JMesh {
        let mut balls = Vec::new();
        let mut springs = Vec::new();

        // Spacing between balls
        let spacing_x = size_x as f32 / subdivision_x as f32;
        let spacing_y = size_y as f32 / subdivision_y as f32;

        // Create balls in a grid
        for i in 0..=subdivision_x {
        for j in 0..=subdivision_y {
            balls.push(Ball {
                position: Vector2 {
                    x: cube_position.x + i as f32 * spacing_x,
                    y: cube_position.y + j as f32 * spacing_y
                },
                velocity: initial_velocity,
                acceleration: Vector2::zero(),
                mass: mass_balls,
            });
        }
    }

        // Create springs based on the spring_method
        match spring_method {
            1 => {
                // Grid
                for i in 0..=subdivision_x {
                    for j in 0..=subdivision_y {
                        if i < subdivision_x {
                            // Horizontal spring
                            springs.push(create_spring(i, j, i + 1, j, spacing_x, subdivision_y));
                        }
                        if j < subdivision_y {
                            // Vertical spring
                            springs.push(create_spring(i, j, i, j + 1, spacing_y, subdivision_y));
                        }
                    }
                }
            }
            2 => {
                // Z pattern springs
                for i in 0..=subdivision_x {
                    for j in 0..=subdivision_y {
                        if i < subdivision_x {
                            // Horizontal spring
                            springs.push(create_spring(i, j, i + 1, j, spacing_x, subdivision_y));
                        }
                        if j < subdivision_y {
                            // Vertical spring
                            springs.push(create_spring(i, j, i, j + 1, spacing_y, subdivision_y));
                        }
                        if i < subdivision_x && j < subdivision_y {
                            // Diagonal spring (down-right)
                            springs.push(create_spring(
                                i,
                                j,
                                i + 1,
                                j + 1,
                                (spacing_x.powi(2) + spacing_y.powi(2)).sqrt(),
                                subdivision_y,
                            ));
                        }
                        if i < subdivision_x && j > 0 {
                            // Diagonal spring (up-right)
                            springs.push(create_spring(
                                i,
                                j,
                                i + 1,
                                j - 1,
                                (spacing_x.powi(2) + spacing_y.powi(2)).sqrt(),
                                subdivision_y,
                            ));
                        }
                    }
                }
            }
            3 => {
                // X pattern springs
                for i in 0..=subdivision_x {
                    for j in 0..=subdivision_y {
                        if i < subdivision_x && j < subdivision_y {
                            // Diagonal spring (down-right)
                            springs.push(create_spring(
                                i,
                                j,
                                i + 1,
                                j + 1,
                                (spacing_x.powi(2) + spacing_y.powi(2)).sqrt(),
                                subdivision_y,
                            ));
                        }
                        if i < subdivision_x && j > 0 {
                            // Diagonal spring (up-right)
                            springs.push(create_spring(
                                i,
                                j,
                                i + 1,
                                j - 1,
                                (spacing_x.powi(2) + spacing_y.powi(2)).sqrt(),
                                subdivision_y,
                            ));
                        }
                    }
                }
            }
            _ => {}
        }

        JMesh { balls, springs }
    }

    // Helper function to create a spring
    fn create_spring(
        i1: usize,
        j1: usize,
        i2: usize,
        j2: usize,
        rest_length: f32,
        subdivision_y: usize,
    ) -> Spring {
        Spring {
            ball1_index: i1 * (subdivision_y + 1) + j1,
            ball2_index: i2 * (subdivision_y + 1) + j2,
            rest_length,
            stiffness: STIFFNESS_COEFF, // Adjust stiffness as needed
        }
    }
}
