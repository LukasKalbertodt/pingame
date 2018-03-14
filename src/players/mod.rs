use {Oracle, PinState};

mod human;
mod random;
mod stepper;

pub use self::human::Human;
pub use self::random::Random;
pub use self::stepper::Stepper;


/// Represents something that can solve the game.
pub trait Player {
    /// Plays one complete game with a given oracle.
    ///
    /// The oracle knows the secret pin state to be guessed. The player has
    /// to repeatedly guess pin states and ask the oracle about the guess in
    /// order to obtain information about the secret. Once the player thinks
    /// it has found the solution, it returns the solution. The player can
    /// also just give up by returning `None`.
    ///
    /// Important: the player may not change state in this method, as the same
    /// player instance might be used for multiple games.
    fn play(&self, oracle: &Oracle) -> Option<PinState>;
}
