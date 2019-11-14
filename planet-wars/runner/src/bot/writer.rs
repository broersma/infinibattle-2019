use super::Bot;

use engine::Universe;
use std::io::{Error, Write};

impl Bot {
    pub fn send_map_info(
        &mut self,
        universe: &Universe,
        player_id: u32,
        seed: i32,
    ) -> Result<(), Error> {
        /* game information */
        writeln!(self.writer, "seed {}", seed)?;

        /* player information */
        writeln!(self.writer, "num-players {}", universe.num_players)?;
        writeln!(self.writer, "player-id {}", player_id)
    }

    pub fn send_game_state(&mut self, universe: &Universe) -> Result<(), Error> {
        writeln!(self.writer, "turn-init")?;

        self.write_planets(universe)?;
        self.write_ships(universe)?;

        writeln!(self.writer, "turn-start")
    }

    pub fn finish_game(&mut self) -> Result<(), Error> {
        writeln!(self.writer, "game-end")
    }

    fn write_planets(&mut self, universe: &Universe) -> Result<(), Error> {
        writeln!(self.writer, "num-planets {}", universe.planets.len())?;
        for planet in &universe.planets {
            let owner = match planet.owner {
                Some(player) => player.to_string(),
                None => "neutral".to_string(),
            };

            writeln!(
                self.writer,
                "planet {} {} {} {} {} {}",
                planet.id, planet.pos.x, planet.pos.y, planet.radius, owner, planet.health
            )?;

            writeln!(
                self.writer,
                "neighbors{}",
                planet
                    .neighbors
                    .iter()
                    .fold(String::new(), |acc, &n| acc + " " + &n.to_string()),
            )?;
        }
        Ok(())
    }

    fn write_ships(&mut self, universe: &Universe) -> Result<(), Error> {
        writeln!(self.writer, "num-ships {}", universe.ships.len())?;
        for ship in &universe.ships {
            writeln!(
                self.writer,
                "ship {} {} {} {} {}",
                ship.pos.x,
                ship.pos.y,
                ship.target_id,
                ship.owner.to_string(),
                ship.power
            )?;
        }

        Ok(())
    }
}
