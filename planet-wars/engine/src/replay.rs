use crate::{prelude::*, Pos, Universe};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Clone, Serialize, Deserialize)]
pub struct Planet {
    pub pos: Pos,
    pub radius: f32,
    pub neighbors: Vec<PlanetId>,
    pub owner: Option<PlayerId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    SendShip(f32, PlanetId, PlanetId),
    Log(String),
    Error(String),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Info {
    pub seed: i32,
    pub map_width: f32,
    pub map_height: f32,
    pub bot_names: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Stats {
    pub winner: Option<usize>,
    pub game_result: String,
    pub turns: usize,
	pub score_bot1: f32,
	pub score_bot2: f32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Replay {
    pub info: Info,
    pub stats: Stats,
    pub planets: Vec<Planet>,
    pub events: Vec<Vec<Vec<Event>>>, // For each turn, for each player, a list of events
}

impl Replay {
    pub fn new(
        universe: &Universe,
        info: Info,
        events: Vec<Vec<Vec<Event>>>,
        game_result: String,
        winner: Option<PlayerId>
    ) -> Self {
        Self {
            info,
            stats: Stats {
                game_result,
                winner,
                turns: events.len(),
			    score_bot1: {
					let bot_id = 0;

					let planet_health = universe.planets.iter().filter(|&p| p.owner == Some(bot_id)).fold(0f32, |acc, p| acc + p.health);
					let ship_power = universe.ships.iter().filter(|&s| s.owner == bot_id).fold(0f32, |acc, s| acc + s.power);
					
					let total_planet_health = universe.planets.iter().fold(0f32, |acc, p| acc + p.health);
					let total_ship_power = universe.ships.iter().fold(0f32, |acc, s| acc + s.power);
					
					let score = planet_health + ship_power;
					let total_score = total_planet_health + total_ship_power;
					
					score / total_score
				},
			    score_bot2: {
					let bot_id = 1;

					let planet_health = universe.planets.iter().filter(|&p| p.owner == Some(bot_id)).fold(0f32, |acc, p| acc + p.health);
					let ship_power = universe.ships.iter().filter(|&s| s.owner == bot_id).fold(0f32, |acc, s| acc + s.power);
					
					let total_planet_health = universe.planets.iter().fold(0f32, |acc, p| acc + p.health);
					let total_ship_power = universe.ships.iter().fold(0f32, |acc, s| acc + s.power);
					
					let score = planet_health + ship_power;
					let total_score = total_planet_health + total_ship_power;
					
					score / total_score
				},
            },
            planets: universe
                .planets
                .iter()
                .map(|p| Planet {
                    pos: p.pos,
                    radius: p.radius,
                    neighbors: p.neighbors.clone(),
					owner: p.owner,
                })
                .collect(),
            events,
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        file.write_all(serde_json::to_string(self)?.as_bytes())
    }
}
