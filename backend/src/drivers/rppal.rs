/// A module to interface Raspberry Pi peripherals using the `rppal` crate.
///

pub mod gpio {

    use crate::drivers::gpio as dgpio;
    use crate::error::Error;
    use rppal::gpio as rgpio;

    /// A GPIO output pin.
    pub struct GpioOutputPin {
        /// The `rppal` output pin.
        pin: rgpio::OutputPin,
    }

    impl GpioOutputPin {
        /// Returns a [GpioOutputPin] instance.
        ///
        /// # Arguments
        ///
        /// * `pin` - the number of the pin to use
        pub fn new(pin: u8) -> Result<Self, Error> {
            match rgpio::Gpio::new() {
                Err(err) => Err(Error::new_with_source(
                    "Failed to acquire rppal Gpio instance.",
                    err,
                )),
                Ok(instance) => match instance.get(pin) {
                    Err(err) => Err(Error::new_with_source(
                        &format!("Failed to acquire pin {}", pin),
                        err,
                    )),
                    Ok(pin) => Ok(Self {
                        pin: pin.into_output(),
                    }),
                },
            }
        }
    }

    impl dgpio::OutputPin for GpioOutputPin {
        fn set(&mut self, state: dgpio::State) {
            match state {
                dgpio::State::High => self.pin.set_high(),
                dgpio::State::Low => self.pin.set_low(),
            }
        }
    }
}

pub mod spi {

    use crate::drivers::spi as dspi;
    use crate::error::Error;
    use rppal::spi as rspi;
    /// Raspberry Pi SPI Bus configurations.
    pub enum Bus {
        Spi0,
        Spi1,
        Spi2,
        Spi3,
        Spi4,
        Spi5,
        Spi6,
    }

    /// Chip select pins
    pub enum Ce {
        Ce0,
        Ce1,
        Ce2,
    }

    /// A SP Interface.
    pub struct Spi {
        /// The `rppal` SPI
        spi: rspi::Spi,
    }

    impl Spi {
        /// Returns a [Spi] instance.
        ///
        /// # Arguments
        ///
        /// * `bus` - SPI bus to use
        /// * `ce` - Chip enable pin to use
        /// * `mode` - SPI mode
        /// * `freq` - clock frequency
        ///
        /// # See Also
        ///
        /// * [pinout.xyz](https://pinout.xyz/)
        pub fn new(bus: Bus, ce: Ce, mode: dspi::Mode, freq: u32) -> Result<Self, Error> {
            let bus = match bus {
                Bus::Spi0 => rppal::spi::Bus::Spi0,
                Bus::Spi1 => rppal::spi::Bus::Spi1,
                Bus::Spi2 => rppal::spi::Bus::Spi2,
                Bus::Spi3 => rppal::spi::Bus::Spi3,
                Bus::Spi4 => rppal::spi::Bus::Spi4,
                Bus::Spi5 => rppal::spi::Bus::Spi5,
                Bus::Spi6 => rppal::spi::Bus::Spi6,
            };
            let ce = match ce {
                Ce::Ce0 => rppal::spi::SlaveSelect::Ss0,
                Ce::Ce1 => rppal::spi::SlaveSelect::Ss1,
                Ce::Ce2 => rppal::spi::SlaveSelect::Ss2,
            };
            let mode = match mode {
                dspi::Mode::Mode0 => rspi::Mode::Mode0,
                dspi::Mode::Mode1 => rspi::Mode::Mode1,
                dspi::Mode::Mode2 => rspi::Mode::Mode2,
                dspi::Mode::Mode3 => rspi::Mode::Mode3,
            };
            match rspi::Spi::new(bus, ce, freq, mode) {
                Err(err) => Err(Error::new_with_source(
                    "Failed to acquire rppal SPI instance.",
                    err,
                )),
                Ok(spi) => Ok(Self { spi }),
            }
        }
    }

    impl dspi::Spi for Spi {
        fn read(&mut self, n: usize) -> Result<Vec<u8>, Error> {
            let mut buf = vec![0; n];
            match self.spi.read(&mut buf) {
                Ok(_) => Ok(buf),
                Err(err) => Err(Error::new_with_source("Failed to read from SPI.", err)),
            }
        }
    }
}
