#[link(name = "wiringpi", kind = "static")]
extern "C" {
    pub fn lcdInit (
        rows: ::libc::c_int,
        cols: ::libc::c_int,
        bits: ::libc::c_int,
        rs: ::libc::c_int,
        strb: ::libc::c_int,
        d0: ::libc::c_int,
        d1: ::libc::c_int,
        d2: ::libc::c_int,
        d3: ::libc::c_int,
        d4: ::libc::c_int,
        d5: ::libc::c_int,
        d6: ::libc::c_int,
        d7: ::libc::c_int) -> ::libc::c_int;

    pub fn lcdHome(fd: ::libc::c_int) -> ();

    pub fn lcdPosition(fd: ::libc::c_int,
                       x: ::libc::c_int,
                       y: ::libc::c_int) -> ();

    pub fn lcdPutchar(fd: ::libc::c_int,
                      data: ::libc::c_char) -> ();


    pub fn lcdPuts(fd: ::libc::c_int,
                   string: *const ::libc::c_char) -> ();
}
