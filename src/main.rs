extern crate term_painter;
extern crate rand;
#[macro_use]
extern crate rand_derive;

mod gen;
mod oracle;
mod players;

pub use oracle::{Eval, Oracle};
use players::Player;
use gen::Generator;


#[derive(Debug, Clone, Copy, PartialEq, Rand)]
#[allow(dead_code)]
pub enum Color {
    Blue,
    Green,
    Yellow,
    Magenta,
    Red,
    Cyan,
}

pub const ALL_COLORS: [Color; 6] = [
    Color::Blue,
    Color::Green,
    Color::Yellow,
    Color::Magenta,
    Color::Red,
    Color::Cyan,
];

#[derive(Debug, Clone, Copy, PartialEq, Rand)]
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

    let player = match args.get(2).map(|s| s.as_ref()).unwrap_or("human") {
        "human" => || Box::new(players::Human::new()) as Box<Player>,
        name => {
            println!("No player called '{}' is available", name);
            return;
        }
    };

    let generator = match args.get(3).map(|s| s.as_ref()).unwrap_or("random") {
        "elisa" => Box::new(gen::Elisa) as Box<Generator>,
        "random" => Box::new(gen::Random) as Box<Generator>,
        "random1" => Box::new(gen::Random1) as Box<Generator>,
        "random2" => Box::new(gen::Random2) as Box<Generator>,
        "random3" => Box::new(gen::Random3) as Box<Generator>,
        name => {
            println!("No generator called '{}' is available", name);
            return;
        }
    };

    match mode {
        Mode::Play => {
            let correct = generator.gen();
            println!("{}", correct);
            play(correct, &*player());
        }
        Mode::Bench => {
            bench(&*generator, player);
        }
    }
}


fn play(correct: PinState, player: &Player) {
    let o = Oracle::new(correct);
    let res = player.play(&o);
    println!("Correct answer was: {}", correct);
    match res {
        None => println!("Player gave up :("),
        Some(res) if res == correct => println!("Yeah :)"),
        _ => println!("Incorrect answer :/"),
    }
}

fn bench<F>(generator: &Generator, mut player: F)
where
    F: FnMut() -> Box<Player>,
{
    let mut num_gave_up = 0;
    let mut num_wins = 0;
    let mut num_incorrect = 0;
    let mut evals = vec![];

    for _ in 0..100 {
        let player = player();
        let correct = generator.gen();

        let o = Oracle::new(correct);
        let res = player.play(&o);
        match res {
            None => num_gave_up += 1,
            Some(res) if res == correct => num_wins += 1,
            _ => num_incorrect += 1,
        }
        evals.push(o.num_evals());
    }

    println!("number won: {}", num_wins);
    println!("give ups: {}", num_gave_up);
    println!("number icorrect: {}", num_incorrect);

    let avg_evals = evals.iter()
        .map(|&n| n as f64)
        .sum::<f64>() / evals.len() as f64;
    println!("avg num evals: {}", avg_evals);
}
