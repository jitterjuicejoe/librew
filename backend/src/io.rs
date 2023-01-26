/// Input/Output interfaces
use super::error::Error;

/// Trait to offer retrieving an input
pub trait Input<T> {
    /// Gets the current value of the input
    fn get(&mut self) -> Result<T, Error>;
}
