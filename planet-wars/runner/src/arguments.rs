use clap::{value_t, App, Arg};
use engine::Pos;
use rand::Rng;
use std::path::PathBuf;

pub struct Arguments {
    pub map_size: Pos,
    pub bot_commands: Vec<PathBuf>,
    pub bot_args: Vec<Vec<String>>,
    pub bot_names: Vec<String>,
    pub seed: i32,
    pub output_file: PathBuf,
}

impl Arguments {
    pub fn new(args: impl Iterator<Item = String>) -> Self {
        let matches = App::new("Planets")
            .version("1.0")
            .author("Wilco Kusee <wilcokusee@gmail.com>")
            .about("Infinibattle game runner")
            .arg(
                Arg::with_name("width")
                    .short("w")
                    .long("width")
                    .help("Sets the game width")
                    .default_value("800"),
            )
            .arg(
                Arg::with_name("height")
                    .short("h")
                    .long("height")
                    .help("Sets the game height")
                    .default_value("600"),
            )
            .arg(
                Arg::with_name("seed")
                    .short("s")
                    .long("seed")
                    .help("Sets the RNG seed")
                    .takes_value(true)
                    .allow_hyphen_values(true),
            )
            .arg(
                Arg::with_name("output file")
                    .short("o")
                    .long("output")
                    .help("File path to save the replay to")
                    .default_value("replay.json"),
            )
            .arg(Arg::with_name("bot 1").index(1).required(true))
            .arg(Arg::with_name("bot 2").index(2).required(true))
            .arg(Arg::with_name("bot 1 args").long("args1").takes_value(true).allow_hyphen_values(true).multiple(true).value_terminator(";").required(true))
            .arg(Arg::with_name("bot 2 args").long("args2").takes_value(true).allow_hyphen_values(true).multiple(true).value_terminator(";").required(true))
            .get_matches_from(args);

        Self {
            map_size: Pos::new(
                value_t!(matches, "width", f32).unwrap(),
                value_t!(matches, "height", f32).unwrap(),
            ),
            bot_commands: vec![
                PathBuf::from(matches.value_of("bot 1").unwrap()),
                PathBuf::from(matches.value_of("bot 2").unwrap()),
            ],
            bot_args: vec![
                matches.values_of("bot 1 args").unwrap().map(|s| s.to_string()).collect(),
                matches.values_of("bot 2 args").unwrap().map(|s| s.to_string()).collect(),
            ],
            bot_names: vec![String::from("Bot 1"), String::from("Bot 2")], // TODO get bot names from commandline
            seed: value_t!(matches, "seed", i32).unwrap_or(rand::thread_rng().gen()),
            output_file: PathBuf::from(matches.value_of("output file").unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bot_commands() {
        let args = test_args(&["path/to/bot.dll", "../../the_bot.dll", "--args1", "-n","bladiebla.pdf", ";", "--args2", "--neuralnet", "woop.nn"]);
        assert_eq!(
            args.bot_commands,
            vec![
                PathBuf::from("path/to/bot.dll"),
                PathBuf::from("../../the_bot.dll")
            ]
        );
        assert_eq!(args.map_size, Pos::new(800.0, 600.0));
        assert_eq!(args.output_file, PathBuf::from("replay.json"));
        assert_eq!(args.bot_args, vec![vec!["-n","bladiebla.pdf"], vec!["--neuralnet","woop.nn"]]);
    }

    #[test]
    fn accepts_arguments() {
        let args = test_args(&[
            "--seed",
            "12345",
            "--width",
            "480",
            "path1.dll",
            "--height",
            "320",
            "path2.py",
            "--args1", 
            "-n","bladiebla.pdf", ";",
            "--output",
            "path3","--args2", 
            "--neuralnet", "woop.nn",
        ]);
        assert_eq!(
            args.bot_commands,
            [PathBuf::from("path1.dll"), PathBuf::from("path2.py")]
        );
        assert_eq!(args.seed, 12345);
        assert_eq!(args.map_size, Pos::new(480.0, 320.0));
        assert_eq!(args.output_file, PathBuf::from("path3"));
        assert_eq!(args.bot_args, vec![vec!["-n","bladiebla.pdf"], vec!["--neuralnet","woop.nn"]]);
    }

    #[test]
    fn accepts_shorthands() {
        let args = test_args(&[
            "-s",
            "2",
            "path1.py",
            "-h",
            "1080",
            "path2.dll",
            "--args1", 
            "-n","bladiebla.pdf", ";",
            "-w",
            "1920",
            "-o",
            "path3","--args2", 
             "--neuralnet", "woop.nn",
        ]);
        assert_eq!(
            args.bot_commands,
            [PathBuf::from("path1.py"), PathBuf::from("path2.dll")]
        );
        assert_eq!(args.seed, 2);
        assert_eq!(args.map_size, Pos::new(1920.0, 1080.0));
        assert_eq!(args.output_file, PathBuf::from("path3"));
        assert_eq!(args.bot_args, vec![vec!["-n","bladiebla.pdf"], vec!["--neuralnet","woop.nn"]]);
    }

    #[test]
    fn negative_seed() {
        let args = test_args(&["dotnet path1", "dotnet path2", "-s", "-123",
            "--args1", "-n","bladiebla.pdf", ";", "--args2", "--neuralnet", "woop.nn",]);
        assert_eq!(args.seed, -123);
    }

    #[test]
    fn args_without_spaces() {
        let args = test_args(&["dotnet bot1", "dotnet bot2", "-h1000", "-s-3",
            "--args1", "-n","bladiebla.pdf", ";", "--args2", "--neuralnet", "woop.nn",]);
        assert_eq!(args.map_size.y, 1000.0);
        assert_eq!(args.seed, -3);
    }

    fn test_args(args_string: &[&str]) -> Arguments {
        let mut args = vec!["program_name"];
        args.extend(args_string);
        Arguments::new(args.iter().map(|s| s.to_string()))
    }

}
