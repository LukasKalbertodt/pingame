use {PinState};
use Color::*;

pub trait Generator {
    fn gen(&self) -> PinState;
}


pub struct Elisa;

impl Generator for Elisa {
    fn gen(&self) -> PinState {
        PinState::new([Cyan, Green, Yellow, Red])
    }
}
