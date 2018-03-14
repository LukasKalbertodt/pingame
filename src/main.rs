extern crate term_painter;

mod gen;
mod oracle;
mod players;

pub use oracle::{Eval, Oracle};
use players::Player;
use gen::Generator;


#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Color {
    Blue,
    Green,
    Yellow,
    Magenta,
    Red,
    Cyan,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PinState {
    pins: [Color; 4],
}

impl PinState {
    pub fn new(pins: [Color; 4]) -> Self {
        Self { pins }
    }
}


fn main() {
    enum Mode {
        Play,
        Bench,
    }

    let args: Vec<_> = std::env::args().collect();
    let mode = match args.get(1).map(|s| s.as_ref()) {
        None => {
            println!("Please specify the game mode: 'bench' or 'play'");
            return;
        }
        Some("play") => Mode::Play,
        Some("bench") => Mode::Bench,
        _ => {
            println!("Invalid game mode: only 'bench' or 'play' are allowed");
            return;
        }
    };

    let player = match args.get(2).map(|s| s.as_ref()) {
        None => {
            println!("Please specify player as second argument");
            return;
        }
        Some("human") => Box::new(players::Human::new()) as Box<Player>,
        Some(name) => {
            println!("No player called '{}' is available", name);
            return;
        }
    };

    let generator = match args.get(3).map(|s| s.as_ref()).unwrap_or("elisa") {
        "elisa" => Box::new(gen::Elisa) as Box<Generator>,
        name => {
            println!("No generator called '{}' is available", name);
            return;
        }
    };

    match mode {
        Mode::Play => {
            let correct = generator.gen();
            play(correct, &*player);
        }
        Mode::Bench => {
            unimplemented!()
        }
    }
}


fn play(correct: PinState, player: &Player) {
    let o = Oracle::new(correct);
    let res = player.play(&o);
    match res {
        None => println!("Player gave up :("),
        Some(res) if res == correct => println!("Yeah :)"),
        _ => println!("Incorrect answer :/"),
    }
}
