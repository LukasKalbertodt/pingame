#![allow(dead_code)]

extern crate term_painter;

mod oracle;
mod players;

pub use oracle::{Eval, Oracle};


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


pub trait Player {
    fn play(&self, oracle: &Oracle) -> PinState;
}



fn main() {
    use Color::*;

    let human = players::Human::new();
    play(PinState::new([Cyan, Green, Yellow, Red]), &human);
}


fn play(correct: PinState, player: &Player) {
    let o = Oracle::new(correct);
    let res = player.play(&o);
    if res == correct {
        println!("Yeah :)");
    } else {
        println!("Baaad!");
    }
}
