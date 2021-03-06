extern crate wiringpi;

use wiringpi::pin::Value::{High, Low};
use wiringpi::time::delay;

fn main() {
    //Setup WiringPi with its own pin numbering order
    let pi = wiringpi::setup();

    //Use WiringPi pin 0 as output
    let pin = pi.output_pin(0);

    loop {
        //Set pin 0 to high and wait one second
        pin.digital_write(High);
        delay(1000);

        //Set pin 0 to low and wait one second
        pin.digital_write(Low);
        delay(1000);
    }
}