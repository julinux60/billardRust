pub mod constant {
    pub const BALL_RADIUS: i32 = 5;

    pub const SCREEN_WIDTH: i32 = 1000;
    pub const SCREEN_HEIGHT: i32 = 800;

    pub const UPDATES_PER_SECOND: u64 = 120;
    pub const UPDATE_TIME_STEP: f32 = 1.0 / UPDATES_PER_SECOND as f32;

    pub const CHOC_RESTITUTION: f32 = 1.0;
    pub const DAMPING_COEFF: f32 = 0.9;
    pub const STIFFNESS_COEFF: f32 = 0.5;
}
