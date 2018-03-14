use {Oracle, PinState};

mod human;

pub use self::human::Human;



pub trait Player {
    fn play(&self, oracle: &Oracle) -> Option<PinState>;
}
