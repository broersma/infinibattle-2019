use super::Bot;
use engine::replay::Event;

use if_chain::if_chain;
use std::time::Duration;

impl Bot {
    pub fn get_commands(&self) -> Result<Vec<Event>, String> {
        let mut events = vec![];

        loop {
            let line = self.read_bot_line()?;
            if line == "end-turn" {
                return Ok(events);
            }

            // Logging
            if line.starts_with('#') {
                //println!("{}", line);
                events.push(Event::Log(line[1..].trim_start().to_string()));
                continue;
            }

            let parts: Vec<&str> = line.split(' ').collect();
            if_chain! {
                if parts.len() == 4;
                if parts[0] == "send-ship";
                if let Ok(power) = parts[1].parse::<f32>();
                if let Ok(from) = parts[2].parse::<usize>();
                if let Ok(to) = parts[3].parse::<usize>();
                then {
                    events.push(Event::SendShip(power, from, to));
                } else {
                    println!("Received invalid command: '{}'", line);
                    events.push(Event::Error(format!("Invalid command: {}", line)));
                }
            }
        }
    }

    fn read_bot_line(&self) -> Result<String, String> {
        self.receiver
            .recv_timeout(Duration::from_millis(1000))
            .map_err(|_| "Bot timed out")?
            .map_err(|err| format!("Error reading bot output: {}", err))
    }
}
