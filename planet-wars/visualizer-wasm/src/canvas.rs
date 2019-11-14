use engine::prelude::SHIP_SPEED;
use engine::{Planet, Pos, Ship, Universe};
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const PLAYER_COLORS: [&str; 2] = ["blue", "red"];
const MIN_SHIP_SIZE: f32 = 1.0;

#[wasm_bindgen]
pub struct Canvas {
    context: web_sys::CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Canvas {
    pub fn new(element: web_sys::Element) -> Self {
        let canvas = element
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Self { context }
    }
}

impl Canvas {
    pub fn render(&self, universe: &Universe, turn_progress: f32) {
        // Clear the canvas
        self.context
            .clear_rect(0.0, 0.0, universe.size.x as f64, universe.size.y as f64);

        self.draw_border(universe.size);

        for planet in &universe.planets {
            self.draw_neighbors(universe, planet);
        }
        for planet in &universe.planets {
            self.draw_planet(planet);
        }
        for ship in &universe.ships {
            self.draw_ship(ship, turn_progress);
        }
    }

    fn draw_border(&self, bottom_right: Pos) {
        self.context.begin_path();
        self.context.move_to(0.0, 0.0);
        self.context.line_to(bottom_right.x as f64, 0.0);
        self.context
            .line_to(bottom_right.x as f64, bottom_right.y as f64);
        self.context.line_to(0.0, bottom_right.y as f64);
        self.context.line_to(0.0, 0.0);
        self.context.stroke();
    }

    fn draw_neighbors(&self, universe: &Universe, planet: &Planet) {
        self.stroke_color("rgb(200, 200, 200)");
        self.context.set_line_width(3.0);
        for neighbor_id in &planet.neighbors {
            if *neighbor_id < planet.id {
                let neighbor = &universe.planets[*neighbor_id];
                self.draw_line(planet.pos, neighbor.pos);
            }
        }
        self.context.stroke();
        self.context.set_line_width(1.0);
    }

    fn draw_planet(&self, planet: &Planet) {
        self.stroke_color("black");
        self.context.begin_path();

        if let Some(owner) = planet.owner {
            self.context
                .arc(
                    planet.pos.x as f64,
                    planet.pos.y as f64,
                    planet.radius as f64,
                    0.0,
                    PI * 2.0,
                )
                .unwrap();
            self.fill(&PLAYER_COLORS[owner as usize]);
        }

        let text_color = if planet.owner.is_some() {
            "white"
        } else {
            "black"
        };
        self.context.set_fill_style(&text_color.into());
        self.draw_circle(planet.pos, planet.radius);

        self.context.set_font("24px serif");
        self.context.set_text_align("center");
        self.context
            .fill_text(
                &(planet.health as u32).to_string(),
                planet.pos.x as f64,
                planet.pos.y as f64,
            )
            .unwrap();
        self.context.stroke();
        self.context.set_fill_style(&"black".into());
    }

    fn draw_ship(&self, ship: &Ship, turn_progress: f32) {
        self.context.begin_path();
        let draw_size = MIN_SHIP_SIZE + (ship.power + 1.0).ln();

        if turn_progress * SHIP_SPEED <= ship.pos.distance_to(ship.target) {
            let pos = ship.position_after_turns(turn_progress);

            self.draw_circle(pos, draw_size);
            self.fill(PLAYER_COLORS[ship.owner as usize]);
            self.stroke_color("rgba(200, 200, 200, 0.5)");
            self.draw_circle(pos, draw_size + 1.0);
            self.context.stroke();
            self.stroke_color("black");
        }
    }
}

// Drawing helper functions
impl Canvas {
    fn draw_circle(&self, pos: Pos, radius: f32) {
        self.context
            .arc(pos.x as f64, pos.y as f64, radius as f64, 0.0, PI * 2.0)
            .unwrap();
    }

    fn draw_line(&self, from: Pos, to: Pos) {
        self.context.move_to(from.x as f64, from.y as f64);
        self.context.line_to(to.x as f64, to.y as f64);
    }

    fn fill(&self, color: &str) {
        self.context.set_fill_style(&color.into());
        self.context.fill();
    }

    fn stroke_color(&self, color: &str) {
        self.context.set_stroke_style(&color.into());
    }
}
