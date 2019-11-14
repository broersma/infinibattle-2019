use rand::Rng;

use super::adjacency_matrix::AdjacencyMatrix;
use crate::prelude::*;
use crate::{Planet, Pos, Replay, Universe};
use std::cmp::Ordering;

const MAX_PLANET_CONNECTIONS: usize = 3;

impl Universe {
    pub fn new(num_players: usize, size: Pos, mut planets: Vec<Planet>) -> Self {
        for i in 0..num_players {
            planets[i].owner = Some(i);
        }

        Self {
            num_players,
            size,
            planets,
            ships: vec![],
        }
    }

    pub fn random<R: Rng>(rng: &mut R, size: Pos, planets_per_player: usize) -> Self {
        loop {
            if let Some(universe) = Self::try_create_random(rng, size, planets_per_player) {
                return universe;
            }
        }
    }

    pub fn load_replay(replay: &Replay) -> Self {
        let num_players = replay.info.bot_names.len();
        let planets = replay
            .planets
            .iter()
            .enumerate()
            .map(|(id, planet)| {
                Planet::new(id, planet.pos, planet.radius, planet.neighbors.clone())
            })
            .collect();
        let map_size = Pos::new(replay.info.map_width, replay.info.map_height);
        Self::new(num_players, map_size, planets)
    }
}

impl Universe {
    fn try_create_random<R: Rng>(
        rng: &mut R,
        size: Pos,
        planets_per_player: usize,
    ) -> Option<Self> {
        let num_players = 2;
        let mut planets: Vec<Planet> = vec![];

        let min = Pos::new(GAME_BORDER_SIZE, GAME_BORDER_SIZE);
        let max = Pos::new(size.x / 2.0, size.y - GAME_BORDER_SIZE);
        for i in 0..planets_per_player {
            loop {
                let planet = if i == 0 {
                    Planet::random(rng, 0, size / 4.0, size / 4.0)
                } else {
                    Planet::random(rng, num_players * i, min, max)
                };

                // Don't spawn planets in the center
                if planet.pos.x >= size.x / 2.0 - PLANET_MIN_DISTANCE
                    && (planet.pos.y - size.y / 2.0) <= PLANET_MIN_DISTANCE
                {
                    continue;
                }

                if planets
                    .iter()
                    .all(|p| p.pos.distance_to(planet.pos) >= PLANET_MIN_DISTANCE)
                {
                    let mut opposite = planet.clone();
                    opposite.id = num_players * i + 1;
                    opposite.pos = size - planet.pos;

                    planets.push(planet);
                    planets.push(opposite);
                    break;
                }
            }
        }

        let distances = AdjacencyMatrix::new(&planets);
        for i in (0..planets_per_player).map(|n| num_players * n) {
            let mut neighbors = vec![];
            for j in 0..planets.len() {
                if let Some(distance) = distances.distance(i, j) {
                    if !planets[i].neighbors.contains(&j) {
                        let penalty = if j % num_players == 0 {
                            // Avoid connecting planet to too many other planets
                            25.0 * planets[j].neighbors.len() as f32
                        } else {
                            // Reward connections to enemy planets
                            -50.0
                        };
                        neighbors.push((distance + penalty, j));
                    }
                }
            }
            neighbors.sort_by(|n1, n2| n1.0.partial_cmp(&n2.0).unwrap_or(Ordering::Less));
            let mut new_neighbors =
                MAX_PLANET_CONNECTIONS - planets[i].neighbors.len().min(MAX_PLANET_CONNECTIONS);
            for (_distance, neighbor) in neighbors.iter() {
                if new_neighbors == 0 {
                    break;
                }
                planets[i].neighbors.push(*neighbor);
                planets[*neighbor].neighbors.push(i);

                if neighbor % num_players == 0 {
                    new_neighbors -= 1;

                    // Connection between own planets, mirror it
                    for j in 1..num_players {
                        planets[i + j].neighbors.push(*neighbor + j);
                        planets[*neighbor + j].neighbors.push(i + j);
                    }
                } else if i / num_players != neighbor / num_players {
                    // Connection between non-mirrored planets, mirror it
                    let neighbor_player = neighbor % num_players;
                    for j in 1..num_players {
                        let mirrored_neighbor = *neighbor + j - neighbor_player - 1;
                        planets[i + j].neighbors.push(mirrored_neighbor);
                        planets[mirrored_neighbor].neighbors.push(i + j);
                    }
                }
            }
        }

        for i in (0..planets_per_player).map(|n| num_players * n) {
            if planets[i].neighbors.iter().any(|n| *n % num_players != 0) {
                return Some(Self::new(num_players, size, planets));
            }
        }
        None
    }
}
