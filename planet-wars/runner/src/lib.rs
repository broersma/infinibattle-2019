// Necessary to make if_chain work
#![recursion_limit = "128"]

pub mod arguments;
mod bot;

use crate::arguments::Arguments;
use crate::bot::*;
use engine::prelude::PlayerId;
use engine::replay::{self, Event, Replay};
use engine::universe::Universe;
use rand::rngs::StdRng;
use rand::SeedableRng;

const MAX_TURNS: u32 = 500;
pub type BotId = usize;

pub struct Runner {
    args: Arguments,
    universe: Universe,
    bots: Vec<Bot>,

    turn: u32,
    events: Vec<Vec<Vec<Event>>>,
}

impl Runner {
    pub fn new(args: Arguments) -> Self {
        // Start bot connections
        let bots: Vec<Bot> = args
            .bot_commands
            .iter()
            .zip(&args.bot_names)
            .zip(&args.bot_args)
            .enumerate()
            .map(|(id, ((path, name), args))| Bot::new(id, name.to_string(), path, args))
            .collect();

        // Start game engine
        let mut rng: StdRng = SeedableRng::seed_from_u64(args.seed as u64);
        println!("Starting the game with seed {}", args.seed);
        let universe = Universe::random(&mut rng, args.map_size, 10);

        Self {
            args,
            universe,
            bots,
            turn: 0,
            events: vec![],
        }
    }

    pub fn run(mut self) -> Result<(), std::io::Error> {
        // Send map info to bots
        for bot in self.bots.iter_mut() {
            let player_id = bot.player_id();
            if let Err(err) = bot.send_map_info(&self.universe, player_id as u32, self.args.seed) {
                return self.save(err.to_string(), Some(1 - player_id));
            }
        }

        // Start the game
        while self.turn < MAX_TURNS {
            self.turn += 1;
            let mut turn_events = vec![];
            for bot in &mut self.bots {
                match Self::bot_turn(bot, &mut self.universe) {
                    Ok(commands) => turn_events.push(commands),
                    Err(err) => {
                        let winner = Some(1 - bot.id);
                        return self.save(err.to_string(), winner);
                    }
                }
            }
            self.events.push(turn_events);

            self.universe.tick();

            if let Some(winner) = self.universe.try_get_winner() {
                return self.save(String::from("Winner"), Some(winner));
            }
        }

        println!("Max turns ({}) reached", MAX_TURNS);
        let winner = self.universe.player_with_most_points();
        self.save(String::from("MaxTurnsReached"), winner)
    }

    fn bot_turn(bot: &mut Bot, universe: &mut Universe) -> Result<Vec<Event>, String> {
        bot.send_game_state(universe).map_err(|err| {
            format!(
                "Error sending game state to '{}': {}",
                bot.display_name, err
            )
        })?;
        let commands = bot.get_commands()?;

        for command in &commands {
            match command {
                &Event::SendShip(power, from, to) => {
                    //println!("Moving ship from {} to {}", from, to);
                    if let Err(e) = universe.try_travel(bot.player_id(), power, from, to) {
                        eprintln!("Error sending ship: {}", e);
                    }
                }
                Event::Log(message) => println!("# {}", message),
                Event::Error(message) => eprintln!(
                    "Error reading command from bot '{}': {}",
                    bot.display_name, message
                ),
            }
        }

        Ok(commands)
    }

    fn save(
        mut self,
        game_result: String,
        winner: Option<PlayerId>,
    ) -> Result<(), std::io::Error> {
        for bot in &mut self.bots {
            // TODO handle error here
            bot.finish_game()?;
        }

        let info = replay::Info {
            seed: self.args.seed,
            map_width: self.args.map_size.x,
            map_height: self.args.map_size.y,
            bot_names: self.args.bot_names,
        };

        let replay = Replay::new(&self.universe, info, self.events, game_result, winner);
        replay.save(&self.args.output_file)
    }
}
