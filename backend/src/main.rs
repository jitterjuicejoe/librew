use std::error::Error;
use std::thread;
use std::time::Duration;

//use librew::gpio::OutputPin;
use librew::drivers::max6675::Max6675;
use librew::drivers::rppal::spi as rspi;
use librew::drivers::spi as dspi;
use librew::io::Input;
// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.

fn main() -> Result<(), Box<dyn Error>> {
    let spi: Box<dyn dspi::Spi> =
        match rspi::Spi::new(rspi::Bus::Spi0, rspi::Ce::Ce0, dspi::Mode::Mode1, 4300000) {
            Err(err) => return Err(Box::new(err)),
            Ok(spi) => Box::new(spi),
        };

    let mut max6675 = Max6675::new(spi);

    loop {
        match max6675.get() {
            Ok(temp) => {
                println!("{}", temp.raw());
            }
            Err(err) => println!("{:?}", err),
        }
        thread::sleep(Duration::from_millis(1000));
    }
    //Ok(())
}
