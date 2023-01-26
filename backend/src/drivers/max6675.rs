use super::spi::Spi;
use crate::error::Error;
use crate::io::Input;
use crate::units::TemperatureKelvin;

/// Driver for MAX6675
///
/// # See Also
///
/// * https://www.analog.com/media/en/technical-documentation/data-sheets/max6675.pdf
pub struct Max6675 {
    spi: Box<dyn Spi>,
}

impl Max6675 {
    pub fn new(spi: Box<dyn Spi>) -> Self {
        Self { spi }
    }
}

impl Input<TemperatureKelvin> for Max6675 {
    fn get(&mut self) -> Result<TemperatureKelvin, Error> {
        match self.spi.read(2) {
            Ok(buf) => {
                let val = ((buf[0] as u16) << 8) | (buf[1] as u16);
                if val & 0x82 != 0 {
                    Err(Error::new("MAX6675 input read from SPI failed validaton."))
                } else if val & 0x4 != 0 {
                    Err(Error::new("MAX6675 thermocouple not attached"))
                } else {
                    Ok(TemperatureKelvin::from((val >> 3) as f64 * 0.25))
                }
            }
            Err(err) => Err(Error::new_with_source("Failed to read from SPI.", err)),
        }
    }
}
