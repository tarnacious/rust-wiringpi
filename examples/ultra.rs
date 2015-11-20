extern crate wiringpi;

use wiringpi::pin::Value::{High, Low};
use wiringpi::time::delay;

fn main() {
    // Example code for a HC-SR04 Ultrasonic Ranging Module
    //
    // http://www.micropik.com/PDF/HCSR04.pdf
    //
    // Assumes 'trigger' is connected to pin 0 and 'echo' to pin 1.
    //
    let pi = wiringpi::setup();

    let trigger = pi.output_pin(0);
    let echo = pi.input_pin(1);

    loop {
        trigger.digital_write(Low);
        delay(1);
        trigger.digital_write(High);
        delay(1);
        trigger.digital_write(Low);

        let mut counter = 0;

        loop {
            match echo.digital_read() {
             High => { break; }
             Low => {}
            }
        }

        loop {
            match echo.digital_read() {
             High => { counter = counter + 1; }
             Low => { break; }
            }
        }
        println!("distance: {}", counter);
    }
}
