use {PinState, Oracle};
use super::Player;

/// A player which makes completely random guesses. As to be expected, its
/// performance is pretty poor.
///
/// If no solution is not found after 50.000 attempts, this player gives up.
pub struct Random;

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
