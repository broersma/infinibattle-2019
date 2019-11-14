use crate::canvas::Canvas;
use engine::prelude::PlayerId;
use engine::replay::Event;
use engine::{Pos, Replay, Universe};
use pcg_rand::Pcg32Basic;
use rand::SeedableRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Visualizer {
    canvas: Canvas,
    universe: Option<Universe>,
    replay: Option<Replay>,
    turn: usize,
    turn_errors: Vec<Vec<(PlayerId, String)>>,
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
impl Visualizer {
    pub fn new(canvas_element: web_sys::Element) -> Self {
        // Setup: log panics to browser console
        console_error_panic_hook::set_once();

        Self {
            canvas: Canvas::new(canvas_element),
            universe: None,
            replay: None,
            turn: 0,
            turn_errors: vec![vec![]],
        }
    }

    pub fn game_length(&mut self) -> i32 {
        if let Some(replay) = &self.replay {
            replay.events.len() as i32
        } else {
            0
        }
    }

    pub fn goto_turn(&mut self, mut turn: i32) {
        if turn < 0 {
            turn = 0;
        }
        if turn > self.game_length() {
            turn = self.game_length();
        }
        if let Some(replay) = self.replay.take() {
            self.universe = Some(Universe::load_replay(&replay));
            self.replay = Some(replay);
            self.turn = 0;
            while self.turn < turn as usize {
                self.tick();
            }
        }
    }

    pub fn start_random(&mut self, width: f32, height: f32, num_planets: usize) {
        self.reset();

        let seed = pcg_rand::seeds::PcgSeeder::seed(js_sys::Date::now() as u64);
        let mut rng = Pcg32Basic::from_seed(seed);

        self.universe = Some(Universe::random(
            &mut rng,
            Pos::new(width, height),
            num_planets,
        ));
    }

    pub fn start_replay(&mut self, replay_string: String) {
        self.reset();

        // TODO error handling
        let replay: Replay = serde_json::from_str(&replay_string).expect("Failed to read replay");

        self.universe = Some(Universe::load_replay(&replay));
        self.replay = Some(replay);
    }

    fn reset(&mut self) {
        self.universe = None;
        self.replay = None;
        self.turn = 0;
        self.turn_errors = vec![vec![]];
    }

    pub fn tick(&mut self) -> bool {
        if !self.can_tick() {
            return false;
        }

        if let Some(universe) = &mut self.universe {
            if let Some(replay) = &self.replay {
                let mut current_turn_errors = vec![];
                for (player_id, player_events) in replay.events[self.turn].iter().enumerate() {
                    for event in player_events {
                        match event {
                            &Event::SendShip(power, from, to) => {
                                if let Err(err) = universe.try_travel(player_id, power, from, to) {
                                    current_turn_errors.push((player_id, err.to_string()));
                                }
                            }
                            Event::Log(_message) => (),
                            Event::Error(_) => (),
                        }
                    }
                }
                self.turn_errors.push(current_turn_errors);
            }
            universe.tick();
            self.turn += 1;
            true
        } else {
            false
        }
    }

    pub fn can_tick(&mut self) -> bool {
        if let Some(replay) = &self.replay {
            return self.turn < replay.events.len();
        }
        false
    }

    pub fn current_turn(&self) -> usize {
        self.turn
    }

    pub fn render(&self, turn_progress: f32) {
        if let Some(universe) = &self.universe {
            self.canvas.render(&universe, turn_progress);
        }
    }

    pub fn populations(&self) -> JsValue {
        let mut counts = [0.0; 2];
        if let Some(universe) = &self.universe {
            for planet in &universe.planets {
                if let Some(owner) = planet.owner {
                    counts[owner] += planet.health;
                }
            }

            for ship in &universe.ships {
                counts[ship.owner] += ship.power;
            }
        }
        JsValue::from_serde(&counts).unwrap()
    }

    pub fn turn_log(&self, turn: usize) -> String {
        let mut output = String::new();
        if let Some(replay) = &self.replay {
            if turn >= replay.events.len() {
                return output;
            }

            if replay.events[turn].iter().any(|events| !events.is_empty()) {
                output.push_str(&format!("--- turn {} ---\n", turn + 1));
            }
            for (player_id, player_events) in replay.events[turn].iter().enumerate() {
                for event in player_events {
                    match event {
                        Event::SendShip(power, from, to) => output.push_str(&format!(
                            "[{}] Sending ship from planet {} to planet {} with power {}\n",
                            player_id, from, to, power
                        )),
                        Event::Log(message) => {
                            output.push_str(&format!("[{}] #{}\n", player_id, message))
                        }
                        Event::Error(error) => {
                            output.push_str(&format!("[{}] Error: {}\n", player_id, error))
                        }
                    }
                }
            }
            if turn > 0 {
                for (player_id, error) in &self.turn_errors[turn - 1] {
                    output.push_str(&format!("[{}] {}\n", player_id, error));
                }
            }
        }
        output
    }
}
