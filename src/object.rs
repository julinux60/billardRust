pub mod object_mod {
    use raylib::prelude::Vector2;
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
}
