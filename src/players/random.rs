use {PinState, Oracle};
use super::Player;

pub struct Random;

impl Random {
    pub fn new() -> Self {
        Random
    }
}


impl Player for Random {
    fn play(&self, oracle: &Oracle) -> Option<PinState> {
        for _ in 0..50_000 {
            let guess: PinState = ::rand::random();
            if oracle.eval_guess(&guess).is_success() {
                return Some(guess);
            }
        }
        None
    }
}
