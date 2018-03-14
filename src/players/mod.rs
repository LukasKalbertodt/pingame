use {Oracle, PinState};

mod human;
mod random;
mod stepper;

pub use self::human::Human;
pub use self::random::Random;
pub use self::stepper::Stepper;



pub trait Player {
    fn play(&self, oracle: &Oracle) -> Option<PinState>;
}
