/// Interface for Serial Peripheral Interface (SPI).
///
// # See also
///
/// * [Serial Peripheral Interface article on Wikipedia](https://en.wikipedia.org/wiki/Serial_Peripheral_Interface)
///
use crate::error::Error;

/// SPI Mode
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Serial_Peripheral_Interface#Mode_numbers)
pub enum Mode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

pub trait Spi {
    /// Returns bytes read from SPI.
    ///
    /// # Arguments
    ///
    /// `n` - number of bytes to read
    ///
    fn read(&mut self, n: usize) -> Result<Vec<u8>, Error>;
}
