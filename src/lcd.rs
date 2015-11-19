extern crate libc;
use std::ffi::CString;
use dev_bindings;

pub fn lcd_init(
    rows: u16,
    cols: u16,
    bits: u16,
    rs: u16,
    strb: u16,
    d0: u16,
    d1: u16,
    d2: u16,
    d3: u16,
    d4: u16,
    d5: u16,
    d6: u16,
    d7: u16) -> i32 {
    return unsafe {
        return dev_bindings::lcdInit(
            rows as libc::c_int,
            cols as libc::c_int,
            bits as libc::c_int,
            rs as libc::c_int,
            strb as libc::c_int,
            d0 as libc::c_int,
            d1 as libc::c_int,
            d2 as libc::c_int,
            d3 as libc::c_int,
            d4 as libc::c_int,
            d5 as libc::c_int,
            d6 as libc::c_int,
            d7 as libc::c_int
        );
    }
}


pub fn lcd_home(fd: i32) -> () {
    unsafe {
        dev_bindings::lcdHome(fd as libc::c_int);
    }
}

pub fn lcd_position(fd: i32,
                   x: u16,
                   y: u16) -> () {
    unsafe {
        dev_bindings::lcdPosition(
            fd as libc::c_int,
            x as libc::c_int,
            y as libc::c_int
        );
    }
}

pub fn lcd_put_char(fd: i32,
                  data: u8) -> () {
    unsafe {
        dev_bindings::lcdPutchar(
            fd as libc::c_int,
            data as libc::c_char,
        );
    }
}


pub fn lcd_puts(fd: i32,
                string: CString) -> () {

    let c_chars = string.as_ptr();
    unsafe {
        dev_bindings::lcdPuts(
            fd as libc::c_int,
            c_chars as *const libc::c_char
        );
    }
}
