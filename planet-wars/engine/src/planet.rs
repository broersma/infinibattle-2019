use crate::{prelude::*, Pos, Ship};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Planet {
    pub id: PlanetId,
    pub pos: Pos,
    pub radius: f32,
    pub owner: Option<PlayerId>,
    pub health: f32,
    pub neighbors: Vec<PlanetId>,
}

impl Planet {
    pub fn new(id: PlanetId, pos: Pos, radius: f32, neighbors: Vec<PlanetId>) -> Self {
        Self {
            id,
            pos,
            radius,
            owner: None,
            health: radius * PLANET_MAX_HEALTH_MULTIPLIER * PLANET_STARTING_HEALTH,
            neighbors,
        }
    }

    pub fn random<R: Rng>(rng: &mut R, id: PlanetId, min: Pos, max: Pos) -> Self {
        let x = rng.gen_range(min.x, max.x + 1.0);
        let y = rng.gen_range(min.y, max.y + 1.0);
        let pos = Pos::new(x, y);
        let radius = rng.gen_range(PLANET_MIN_RADIUS, PLANET_MAX_RADIUS + 1.0);
        Self::new(id, pos, radius, vec![])
    }

    pub fn grow(&mut self) {
        if self.owner.is_some() {
            self.health += self.radius * PLANET_GROWTH_SPEED;
            self.health = self.health.min(PLANET_MAX_HEALTH_MULTIPLIER * self.radius);
        }
    }

    pub fn land_ship(&mut self, ship: &Ship) {
        if self.owner == Some(ship.owner) {
            // Reinforce
            self.health += ship.power;
        } else if ship.power == self.health {
            self.health = 0.0;
            self.owner = None;
        } else if ship.power > self.health {
            self.health = ship.power - self.health;
            self.owner = Some(ship.owner);
        } else {
            self.health -= ship.power;
        }
    }
}

impl PartialEq for Planet {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
