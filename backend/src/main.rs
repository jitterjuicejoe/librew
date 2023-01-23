use std::error::Error;
use std::thread;
use std::time::Duration;

//use librew::gpio::OutputPin;
use librew::drivers::rppal;
use librew::drivers::{spi, spi::Spi};
// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.

fn main() -> Result<(), Box<dyn Error>> {
    let mut spi: Box<dyn Spi> =
        match rppal::Spi::new(rppal::Bus::Spi0, rppal::Ce::Ce0, spi::Mode::Mode3, 4300000) {
            Err(err) => return Err(Box::new(err)),
            Ok(spi) => Box::new(spi),
        };
    loop {
        let buf = spi.read(2)?;
        let temp = ((buf[0] as usize) << 8) | (buf[1] as usize);
        if temp & 0x4 != 0 {
            println!("No thermocouple");
        } else {
            println!("temp: {}", (temp >> 3) as f64 * 0.25);
        }
        thread::sleep(Duration::from_millis(1000));
    }
    //Ok(())
}
