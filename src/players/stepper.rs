use {Color, PinState, Oracle};
use Color::*;
use super::Player;

/// A player which solves each individual digit of the secret state on its own.
/// This player is a bit worse than the average human player, but a lot better
/// than the random player.
pub struct Stepper;


impl Player for Stepper {
    fn play(&self, oracle: &Oracle) -> Option<PinState> {
        let mut guess = PinState::new([Blue, Blue, Blue, Blue]);
        let mut eval = oracle.eval_guess(&guess);

        const OTHER_COLORS: [Color; 5] = [
            Green,
            Yellow,
            Magenta,
            Red,
            Cyan,
        ];

        for d in 0..4 {
            if eval.is_success() {
                break;
            }

            for &c in &OTHER_COLORS {
                let new_guess = {
                    let mut new = guess.clone();
                    new.pins[d] = c;
                    new
                };
                let new_eval = oracle.eval_guess(&new_guess);

                if new_eval.num_black() > eval.num_black() {
                    guess = new_guess;
                    eval = new_eval;
                    break;
                } else if new_eval.num_black() < eval.num_black() {
                    break;
                }
            }
        }

        Some(guess)
    }
}
