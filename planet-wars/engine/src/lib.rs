pub mod planet;
pub mod pos;
pub mod replay;
pub mod ship;
pub mod universe;

pub use planet::Planet;
pub use pos::Pos;
pub use replay::Replay;
pub use ship::Ship;
pub use universe::Universe;

pub mod prelude {
    pub type PlanetId = usize;
    pub type PlayerId = usize;

    pub const PLANET_MIN_RADIUS: f32 = 20.0;
    pub const PLANET_MAX_RADIUS: f32 = 40.0;
    pub const GAME_BORDER_SIZE: f32 = PLANET_MAX_RADIUS;
    pub const PLANET_MIN_DISTANCE: f32 = 2.0 * PLANET_MAX_RADIUS + 5.0;
    pub const PLANET_GROWTH_SPEED: f32 = 0.05;

    pub const PLANET_STARTING_HEALTH: f32 = 0.33;
    pub const PLANET_MAX_HEALTH_MULTIPLIER: f32 = 5.0;

    pub const SHIP_SPEED: f32 = 15.0;
}
