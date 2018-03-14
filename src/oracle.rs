use std::cell::Cell;

use super::PinState;

#[derive(Debug, Clone)]
pub struct Oracle {
    correct: PinState,
    num_evals: Cell<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Eval {
    num_white: u8,
    num_black: u8,
}

impl Eval {
    pub fn is_success(&self) -> bool {
        self.num_black == 4
    }

    pub fn num_black(&self) -> u8 {
        self.num_black
    }

    pub fn num_white(&self) -> u8 {
        self.num_white
    }
}

impl Oracle {
    pub fn new(correct: PinState) -> Self {
        Self {
            correct,
            num_evals: Cell::new(0),
        }
    }

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
    fn simple() {
        let o = Oracle::new(PinState::new([Blue, Red, Yellow, Green]));

        // Everything wrong
        assert_eq!(
            o.eval_guess(&PinState::new([Cyan, Cyan, Magenta, Cyan])),
            Eval {
                num_white: 0,
                num_black: 0,
            }
        );

        // One black
        assert_eq!(
            o.eval_guess(&PinState::new([Blue, Cyan, Blue, Cyan])),
            Eval {
                num_white: 0,
                num_black: 1,
            }
        );

        // One black, one white
        assert_eq!(
            o.eval_guess(&PinState::new([Blue, Cyan, Red, Cyan])),
            Eval {
                num_white: 1,
                num_black: 1,
            }
        );

        // Correct colors, but incorrect position
        assert_eq!(
            o.eval_guess(&PinState::new([Red, Yellow, Green, Blue])),
            Eval {
                num_white: 4,
                num_black: 0,
            }
        );

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
