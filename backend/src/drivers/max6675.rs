use super::spi::Spi;
use crate::error::Error;
use crate::io::Input;
use crate::units::TemperatureCelcius;

/// Driver for MAX6675
///
/// # See Also
///
/// * https://www.analog.com/media/en/technical-documentation/data-sheets/max6675.pdf
pub struct Max6675 {
    spi: Box<dyn Spi>,
}

impl Max6675 {
    pub fn new(spi: impl Spi + 'static) -> Self {
        Self { spi: Box::new(spi) }
    }
}

impl Input<TemperatureCelcius> for Max6675 {
    fn get(&mut self) -> Result<TemperatureCelcius, Error> {
        match self.spi.read(2) {
            Ok(buf) => {
                let val = ((buf[0] as u16) << 8) | (buf[1] as u16);
                // Input validation:
                // The most significant bit should be unset, according to the spec (dummy bit).
                // Bit 2 should also be unset "for chip identification".
                if val & 0x8002 != 0 {
                    Err(Error::new(&format!(
                        "MAX6675 input read from SPI failed validation. {}",
                        val
                    )))
                // Bit 3 is set if thermocouple is open (not attached)
                } else if val & 0x4 != 0 {
                    Err(Error::new("MAX6675 thermocouple not attached"))
                } else {
                    // Temperature readings are given in 1/4 degrees Celcius.
                    // The three leftmost bits are control bits.
                    Ok(TemperatureCelcius::from((val >> 3) as f64 * 0.25))
                }
            }
            Err(err) => Err(Error::new_with_source("Failed to read from SPI.", err)),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::drivers::spi::Spi;
    use crate::error::Error;

    /// Mock implementation of SPI
    struct MockSpi<I>
    where
        I: Iterator<Item = Option<u8>>,
    {
        it: I,
    }

    impl<I> MockSpi<I>
    where
        I: Iterator<Item = Option<u8>>,
    {
        /// Constructs a [MockSpi] instance.
        ///
        /// # Arguments
        ///
        /// * `it` - iterator over the simulated SPI output
        pub fn new(it: I) -> Self {
            Self { it }
        }
    }

    impl<I> Spi for MockSpi<I>
    where
        I: Iterator<Item = Option<u8>>,
    {
        fn read(&mut self, n: usize) -> Result<Vec<u8>, Error> {
            // Output the requested number of bytes. `None` values
            // in the input will be treated as SPI read errors.
            let mut res = vec![];
            for _ in 0..n {
                if let Some(Some(b)) = self.it.next() {
                    res.push(b);
                } else {
                    return Err(Error::new("Mock SPI read error."));
                }
            }

            Ok(res)
        }
    }

    /// Converts a temperature to the corresponding
    /// MAX6675 output.
    ///
    /// # Arguments
    ///
    /// * `t` - temperature in 1/100 degrees Celcius
    fn temp_to_max6675_output(t: usize) -> impl Iterator<Item = Option<u8>> {
        use std::iter::once;
        // The MAX6675 gives the temperature in 1/4 degrees Celcius,
        // left shifted by 3 for control bits.
        let t = ((t * 4) / 100) << 3;
        once(Some((t >> 8) as u8)).chain(once(Some((t & 0xFF) as u8)))
    }

    /// Converts a temperature to the corresponding driver output
    ///
    /// # Arguments
    ///
    /// * `t` - temperature in 1/100 degrees Celcius
    fn temp_to_driver_output(t: usize) -> f64 {
        (t / 25) as f64 * 0.25
    }

    #[test]
    /// Read a single temperature.
    fn read_temp_1() {
        const TEMP: usize = 2500;
        let spi = MockSpi::new(temp_to_max6675_output(TEMP));
        let mut max6675 = Max6675::new(spi);
        assert_eq!(max6675.get().unwrap().raw(), temp_to_driver_output(TEMP));
    }

    #[test]
    // Read multiple temperatures.
    fn read_temp_2() {
        const TEMPS: [usize; 4] = [2500, 2525, 2550, 2575];
        let spi = MockSpi::new(TEMPS.iter().flat_map(|t| temp_to_max6675_output(*t)));
        let mut max6675 = Max6675::new(spi);
        for t in TEMPS {
            assert_eq!(max6675.get().unwrap().raw(), temp_to_driver_output(t));
        }
    }

    #[test]
    // Test validation error for MSB set
    fn msb_validation() {
        let spi = MockSpi::new([Some(0x80), Some(0x00)].into_iter());
        let mut max6675 = Max6675::new(spi);
        assert!(max6675.get().is_err());
    }

    #[test]
    // Test validation error for chip ID bit (bit 2) set
    fn chip_id_validation() {
        let spi = MockSpi::new([Some(0x00), Some(0x02)].into_iter());
        let mut max6675 = Max6675::new(spi);
        assert!(max6675.get().is_err());
    }

    #[test]
    // Test error on thermocouple open
    fn thermocouple_open() {
        let spi = MockSpi::new([Some(0x00), Some(0x04)].into_iter());
        let mut max6675 = Max6675::new(spi);
        assert!(max6675.get().is_err());
    }

    #[test]
    // Test error on SPI read error
    fn spi_read_error() {
        let spi = MockSpi::new([Some(0x00), None].into_iter());
        let mut max6675 = Max6675::new(spi);
        assert!(max6675.get().is_err());
    }

    #[test]
    // Test error on missing SPI data
    fn missing_spi_data() {
        let spi = MockSpi::new([Some(0x00)].into_iter());
        let mut max6675 = Max6675::new(spi);
        assert!(max6675.get().is_err());
    }
}
