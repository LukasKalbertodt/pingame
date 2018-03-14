use {Oracle, PinState};

mod human;
mod random;

pub use self::human::Human;
pub use self::random::Random;



pub trait Player {
    fn play(&self, oracle: &Oracle) -> Option<PinState>;
}
