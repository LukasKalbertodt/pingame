use std::fmt;


use term_painter::{Color as TermColor, ToStyle};

use {PinState, Color, Oracle, Eval};
use super::Player;

pub struct Human;

impl Human {
    pub fn new() -> Self {
        Human
    }
}


impl Player for Human {
    fn play(&self, oracle: &Oracle) -> Option<PinState> {
        loop {
            let guess = read_guess();
            let eval = oracle.eval_guess(&guess);
            if eval.is_success() {
                return Some(guess);
            }

            println!("{}  ⇒   {}", guess, eval);
        }
    }
}


fn read_guess() -> PinState {
    use std::io::{self, Write};

    fn char_to_color(c: char) -> Color {
        match c {
            'b' => Color::Blue,
            'g' => Color::Green,
            'y' => Color::Yellow,
            'm' => Color::Magenta,
            'r' => Color::Red,
            'c' => Color::Cyan,
            _ => panic!(),
        }
    }

    loop {
        print!("Your guess: ");
        io::stdout().flush().unwrap();

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s.make_ascii_lowercase();
        let s = s.trim_right();

        if s.is_empty() {
            continue;
        }

        let input: Vec<_> = s.chars().filter(|c| !c.is_whitespace()).collect();

        const VALID_CHARS: [char; 6] = ['b', 'g', 'y', 'm', 'r', 'c'];

        if !input.iter().all(|c| VALID_CHARS.contains(c)) {
            println!("Error in input: only the follow characters are allowed: {:?}", VALID_CHARS);
            continue;
        }

        if input.len() != 4 {
            println!("Error in input: {} colors are given, 4 expected", s.len());
            continue;
        }

        return PinState::new([
            char_to_color(input[0]),
            char_to_color(input[1]),
            char_to_color(input[2]),
            char_to_color(input[3]),
        ]);
    }
}

impl fmt::Display for Eval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.num_black() {
            write!(f, "{}", TermColor::White.paint("○ "))?;
        }

        for _ in 0..self.num_white() {
            write!(f, "{} ", TermColor::White.paint('●'))?;
        }

        Ok(())
    }
}

impl fmt::Display for PinState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for pin in &self.pins {
            let color = color_to_term_color(pin);
            write!(f, "{} ", color.paint('●'))?;
        }
        Ok(())
    }
}

fn color_to_term_color(c: &Color) -> TermColor {
    match *c {
        Color::Blue => TermColor::Blue,
        Color::Green => TermColor::Green,
        Color::Yellow => TermColor::Yellow,
        Color::Magenta => TermColor::Magenta,
        Color::Red => TermColor::Red,
        Color::Cyan => TermColor::Cyan,
    }
}
