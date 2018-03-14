use rand::{self, Rng, random};

use {ALL_COLORS, Color, PinState};
use Color::*;


/// Represents types able to generate secret pin states.
pub trait Generator {
    fn gen(&self) -> PinState;
}


/// Elisa's pin state of choice. Always the same.
pub struct Elisa;

impl Generator for Elisa {
    fn gen(&self) -> PinState {
        PinState::new([Cyan, Green, Yellow, Red])
    }
}


/// Generates pin states consisting of only one random color.
pub struct Random1;

impl Generator for Random1 {
    fn gen(&self) -> PinState {
        let c: Color = random();
        PinState::new([c, c, c, c])
    }
}

/// Generates pin states with two different colors.
pub struct Random2;

impl Generator for Random2 {
    fn gen(&self) -> PinState {
        let mut v = ALL_COLORS.to_vec();
        rand::thread_rng().shuffle(&mut v);
        let c1 = v[0];
        let c2 = v[1];

        let choose = || if random() { c1 } else { c2 };

        let first_three = [choose(), choose(), choose()];
        let last = if first_three.iter().all(|&c| c == c1) {
            c2
        } else if first_three.iter().all(|&c| c == c2) {
            c1
        } else {
            choose()
        };

        PinState::new([
            first_three[0],
            first_three[1],
            first_three[2],
            last,
        ])
    }
}

/// Generates pin states with three random colors.
pub struct Random3;

impl Generator for Random3 {
    fn gen(&self) -> PinState {
        let mut v = ALL_COLORS.to_vec();
        rand::thread_rng().shuffle(&mut v);
        let c1 = v[0];
        let c2 = v[1];
        let c3 = v[2];

        let doubled = *rand::thread_rng().choose(&[c1, c2, c3]).unwrap();

        let mut arr = [c1, c2, c3, doubled];
        rand::thread_rng().shuffle(&mut arr);

        PinState::new(arr)
    }
}


/// Generates completely random pin states.
pub struct Random;

impl Generator for Random {
    fn gen(&self) -> PinState {
        random()
    }
}
