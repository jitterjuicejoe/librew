/// Interface for General Purpose Input/Output.
///
/// # See Also
///
/// * [Wikipedia page on GPIO](https://en.wikipedia.org/wiki/General-purpose_input%2Foutput)
///

/// Pin state.
pub enum State {
    Low,
    High,
}

/// An output pin.
pub trait OutputPin {
    /// Sets pin state.
    ///
    /// # Arguments
    ///
    /// * `state` - Pin state
    fn set(&mut self, state: State);
}
