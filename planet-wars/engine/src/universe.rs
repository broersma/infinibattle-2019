use crate::{prelude::*, Planet, Pos, Ship};

mod adjacency_matrix;
mod builder;
pub use builder::*;

#[derive(Debug)]
pub struct Universe {
    pub num_players: usize,
    pub size: Pos,
    pub planets: Vec<Planet>,
    pub ships: Vec<Ship>,
}

impl Universe {
    pub fn tick(&mut self) {
        self.move_ships();
        for planet in &mut self.planets {
            planet.grow();
        }
    }

    fn move_ships(&mut self) {
        let mut i = 0;
        while i != self.ships.len() {
            self.ships[i].do_move();

            let ship = &self.ships[i];
            if ship.reached_target() {
                self.planets[ship.target_id].land_ship(ship);
                self.ships.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn planet_at_pos(&mut self, pos: Pos) -> Option<PlanetId> {
        for planet in &self.planets {
            if planet.pos.distance_to(pos) <= planet.radius as f32 {
                return Some(planet.id);
            }
        }
        None
    }

    pub fn try_travel(
        &mut self,
        player_id: PlayerId,
        power: f32,
        from: PlanetId,
        to: PlanetId,
    ) -> Result<(), String> {
        if from == to {
            return Err("Attempt to send a ship to its planet of origin".to_string());
        }

        if from >= self.planets.len() {
            return Err("Attempt to send a ship from non-existent planet".to_string());
        }

        if to >= self.planets.len() {
            return Err("Attempt to send a ship to non-existent planet".to_string());
        }

        if player_id >= self.num_players {
            return Err("Command issued by non-existent player".to_string());
        }

        if self.planets[from].owner != Some(player_id) {
            return Err("Attempt to send a ship from a planet not owned by the player".to_string());
        }

        if !self.planets[from].neighbors.contains(&to) {
            return Err("Attempt to send a ship to a planet that is not a neighbor".to_string());
        }

        if power <= 0.0 {
            return Err("Attempt to send a ship with negative power".to_string());
        }

        if self.planets[from].health - power < 1.0 {
            return Err(format!("Attempt to send a ship (power: {}) leaving the planet (id: {}) with less than 1.0 health (current health: {})", power, from, self.planets[from].health));
        }

        self.travel(player_id, power, from, to);
        Ok(())
    }

    pub fn travel(&mut self, owner: PlayerId, power: f32, from: PlanetId, to: PlanetId) {
        self.planets[from].health -= power as f32;

        self.ships.push(Ship {
            owner,
            power,
            pos: self.planets[from].pos,
            target: self.planets[to].pos,
            target_id: self.planets[to].id,
        });
    }

    pub fn try_get_winner(&self) -> Option<PlayerId> {
        let mut has_planets = vec![false; self.num_players as usize];
        for planet in &self.planets {
            if let Some(owner) = planet.owner {
                has_planets[owner] = true;
            }
        }

        // Game is won if only one player has planets
        if has_planets.iter().filter(|&b| *b).count() == 1 {
            return has_planets.iter().position(|&b| b);
        }
        None
    }

    pub fn player_with_most_points(&self) -> Option<PlayerId> {
        let mut points = vec![0.0; self.num_players];
        for planet in &self.planets {
            if let Some(owner) = planet.owner {
                points[owner] += planet.health;
            }
        }
        for ship in &self.ships {
            points[ship.owner] += ship.power;
        }

        let points = points.iter().map(|p| *p as i32).collect::<Vec<_>>();
        let max_points = points.iter().max().unwrap();
        let winning_players = points
            .iter()
            .enumerate()
            .filter(|(_, points)| *points == max_points)
            .map(|(player_id, _)| player_id)
            .collect::<Vec<_>>();

        if winning_players.len() == 1 {
            Some(winning_players[0])
        } else {
            None
        }
    }
}
