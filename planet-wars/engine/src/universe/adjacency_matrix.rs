use crate::Planet;

const EXTRA_RADIUS: f32 = 1.1;

pub struct AdjacencyMatrix {
    distances: Vec<Vec<Option<f32>>>,
}

impl AdjacencyMatrix {
    pub fn new(planets: &[Planet]) -> Self {
        let mut distances = vec![vec![None; planets.len()]; planets.len()];
        for i in 0..planets.len() {
            for j in i + 1..planets.len() {
                if planets.iter().filter(|p| p.id != i && p.id != j).all(|p| {
                    p.pos
                        .distance_to_line_segment(planets[i].pos, planets[j].pos)
                        >= EXTRA_RADIUS * p.radius
                }) {
                    distances[i][j] = Some(planets[i].pos.distance_to(planets[j].pos));
                }
            }
        }
        Self { distances }
    }

    pub fn distance(&self, i: usize, j: usize) -> Option<f32> {
        if i < j {
            self.distances[i][j]
        } else {
            self.distances[j][i]
        }
    }
}
