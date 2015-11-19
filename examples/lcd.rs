extern crate wiringpi;
use std::ffi::CString;


fn main() {
    wiringpi::setup();

    let bits = 4;
    let cols = 16;
    let rows = 2;

    // Initialized the LCD
    let lcd = wiringpi::lcd::lcd_init(rows, cols, bits, 11, 10, 4, 5, 6, 7, 0, 0, 0, 0);

    // Move the cursore
    wiringpi::lcd::lcd_position(lcd, 1, 1);

    // Write something
    wiringpi::lcd::lcd_puts(lcd, CString::new("Hello World!").unwrap());

    loop {
    }
}
