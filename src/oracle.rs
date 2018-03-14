use std::cell::Cell;

use super::PinState;


/// An evaluation of a guess of the secret pin state.
///
/// This evaluation can only be obtained through the oracle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Eval {
    num_white: u8,
    num_black: u8,
}

impl Eval {
    /// Returns `true` iff the guess was completely correct.
    pub fn is_success(&self) -> bool {
        self.num_black == 4
    }

    /// Returns the number of black pins. Each black pin represents a pin in
    /// the guess that has the correct color and is at the correct position.
    pub fn num_black(&self) -> u8 {
        self.num_black
    }

    /// Returns the number of white pins. Each white pin represents a pin in
    /// the guess that has the correct color, but is in an incorrect position.
    pub fn num_white(&self) -> u8 {
        self.num_white
    }
}


/// This structure is given to the player as the only way to obtain information
/// about the secret pin state.
///
/// It internally counts the number of times it was asked about an evaluation
/// of a guess. This is used as a metric to rate different players.
#[derive(Debug, Clone)]
pub struct Oracle {
    correct: PinState,
    num_evals: Cell<u32>,
}

impl Oracle {
    /// Creates a new oracle which knows about the given secret pin state.
    pub fn new(correct: PinState) -> Self {
        Self {
            correct,
            num_evals: Cell::new(0),
        }
    }

    /// Evaluates the given guess.
    pub fn eval_guess(&self, guess: &PinState) -> Eval {
        self.num_evals.set(self.num_evals.get() + 1);

        let mut num_black = 0;
        let mut num_white = 0;
        let mut remaining = self.correct.pins.to_vec();

        for i in 0..4 {
            let curr = guess.pins[i];
            if curr == self.correct.pins[i] {
                num_black += 1;
            } else if remaining.contains(&curr) {
                num_white += 1;
            }

            if let Some(pos) = remaining.iter().position(|&e| e == curr) {
                remaining.remove(pos);
            }
        }

        Eval { num_black, num_white }
    }

    /// Returns the number of times `eval_guess` was called since creation of
    /// this oracle.
    pub fn num_evals(&self) -> u32 {
        self.num_evals.get()
    }
}


#[cfg(test)]
mod tests {
    use Color::*;
    use PinState;
    use super::{Eval, Oracle};

    #[test]
    fn everything_wrong() {
        let o = Oracle::new(PinState::new([Blue, Red, Yellow, Green]));

        // Everything wrong
        assert_eq!(
            o.eval_guess(&PinState::new([Cyan, Cyan, Magenta, Cyan])),
            Eval {
                num_white: 0,
                num_black: 0,
            }
        );
    }

    #[test]
    fn one_black() {
        let o = Oracle::new(PinState::new([Blue, Red, Yellow, Green]));

        // One black
        assert_eq!(
            o.eval_guess(&PinState::new([Blue, Cyan, Blue, Cyan])),
            Eval {
                num_white: 0,
                num_black: 1,
            }
        );
    }

    #[test]
    fn one_black_one_white() {
        let o = Oracle::new(PinState::new([Blue, Red, Yellow, Green]));

        // One black, one white
        assert_eq!(
            o.eval_guess(&PinState::new([Blue, Cyan, Red, Cyan])),
            Eval {
                num_white: 1,
                num_black: 1,
            }
        );
    }

    #[test]
    fn correct_colors_incorrect_position() {
        let o = Oracle::new(PinState::new([Blue, Red, Yellow, Green]));

        // Correct colors, but incorrect position
        assert_eq!(
            o.eval_guess(&PinState::new([Red, Yellow, Green, Blue])),
            Eval {
                num_white: 4,
                num_black: 0,
            }
        );
    }

    #[test]
    fn everything_correct() {
        let o = Oracle::new(PinState::new([Blue, Red, Yellow, Green]));

        // Everything correct
        assert_eq!(
            o.eval_guess(&PinState::new([Blue, Red, Yellow, Green])),
            Eval {
                num_white: 0,
                num_black: 4,
            }
        );
    }
}
