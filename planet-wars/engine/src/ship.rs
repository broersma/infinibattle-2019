use crate::{prelude::*, Pos};

#[derive(Debug, Clone)]
pub struct Ship {
    pub owner: PlayerId,
    pub power: f32,
    pub pos: Pos,
    pub target: Pos,
    pub target_id: PlanetId,
}

impl Ship {
    pub fn distance_to_target(&self) -> f32 {
        self.pos.distance_to(self.target)
    }

    pub fn do_move(&mut self) {
        self.pos = self.position_after_turns(1.0);
    }

    pub fn position_after_turns(&self, turns: f32) -> Pos {
        let delta = self.pos.unit_direction_to(self.target);
        self.pos + delta * SHIP_SPEED * turns
    }

    pub fn reached_target(&self) -> bool {
        self.distance_to_target() < SHIP_SPEED
    }
}
