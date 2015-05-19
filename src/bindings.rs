#![allow(dead_code)]

/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy)]
#[allow(non_snake_case)]
pub struct Struct_wiringPiNodeStruct {
    pub pinBase: ::libc::c_int,
    pub pinMax: ::libc::c_int,
    pub fd: ::libc::c_int,
    pub data0: ::libc::c_uint,
    pub data1: ::libc::c_uint,
    pub data2: ::libc::c_uint,
    pub data3: ::libc::c_uint,
    pub pinMode: ::std::option::Option<extern "C" fn(node:
                                                         *mut Struct_wiringPiNodeStruct,
                                                     pin: ::libc::c_int,
                                                     mode: ::libc::c_int)
                                           -> ()>,
    pub pullUpDnControl: ::std::option::Option<extern "C" fn(node:
                                                                 *mut Struct_wiringPiNodeStruct,
                                                             pin:
                                                                 ::libc::c_int,
                                                             mode:
                                                                 ::libc::c_int)
                                                   -> ()>,
    pub digitalRead: ::std::option::Option<extern "C" fn(node:
                                                             *mut Struct_wiringPiNodeStruct,
                                                         pin: ::libc::c_int)
                                               -> ::libc::c_int>,
    pub digitalWrite: ::std::option::Option<extern "C" fn(node:
                                                              *mut Struct_wiringPiNodeStruct,
                                                          pin: ::libc::c_int,
                                                          value:
                                                              ::libc::c_int)
                                                -> ()>,
    pub pwmWrite: ::std::option::Option<extern "C" fn(node:
                                                          *mut Struct_wiringPiNodeStruct,
                                                      pin: ::libc::c_int,
                                                      value: ::libc::c_int)
                                            -> ()>,
    pub analogRead: ::std::option::Option<extern "C" fn(node:
                                                            *mut Struct_wiringPiNodeStruct,
                                                        pin: ::libc::c_int)
                                              -> ::libc::c_int>,
    pub analogWrite: ::std::option::Option<extern "C" fn(node:
                                                             *mut Struct_wiringPiNodeStruct,
                                                         pin: ::libc::c_int,
                                                         value: ::libc::c_int)
                                               -> ()>,
    pub next: *mut Struct_wiringPiNodeStruct,
}
impl ::std::clone::Clone for Struct_wiringPiNodeStruct {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_wiringPiNodeStruct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[link(name = "wiringpi", kind = "static")]
extern "C" {
    pub static mut piModelNames: [*const ::libc::c_char; 7usize];
    pub static mut piRevisionNames: [*const ::libc::c_char; 5usize];
    pub static mut piMakerNames: [*const ::libc::c_char; 5usize];
    pub static mut wiringPiNodes: *mut Struct_wiringPiNodeStruct;
}
#[link(name = "wiringpi", kind = "static")]
extern "C" {
    pub fn wiringPiFailure(fatal: ::libc::c_int,
                           message: *const ::libc::c_char, ...)
     -> ::libc::c_int;
    pub fn wiringPiFindNode(pin: ::libc::c_int)
     -> *mut Struct_wiringPiNodeStruct;
    pub fn wiringPiNewNode(pinBase: ::libc::c_int, numPins: ::libc::c_int)
     -> *mut Struct_wiringPiNodeStruct;
    pub fn wiringPiSetup() -> ::libc::c_int;
    pub fn wiringPiSetupSys() -> ::libc::c_int;
    pub fn wiringPiSetupGpio() -> ::libc::c_int;
    pub fn wiringPiSetupPhys() -> ::libc::c_int;
    pub fn pinModeAlt(pin: ::libc::c_int, mode: ::libc::c_int) -> ();
    pub fn pinMode(pin: ::libc::c_int, mode: ::libc::c_int) -> ();
    pub fn pullUpDnControl(pin: ::libc::c_int, pud: ::libc::c_int) -> ();
    pub fn digitalRead(pin: ::libc::c_int) -> ::libc::c_int;
    pub fn digitalWrite(pin: ::libc::c_int, value: ::libc::c_int) -> ();
    pub fn pwmWrite(pin: ::libc::c_int, value: ::libc::c_int) -> ();
    pub fn analogRead(pin: ::libc::c_int) -> ::libc::c_int;
    pub fn analogWrite(pin: ::libc::c_int, value: ::libc::c_int) -> ();
    pub fn wiringPiSetupPiFace() -> ::libc::c_int;
    pub fn wiringPiSetupPiFaceForGpioProg() -> ::libc::c_int;
    pub fn piBoardRev() -> ::libc::c_int;
    pub fn piBoardId(model: *mut ::libc::c_int, rev: *mut ::libc::c_int,
                     mem: *mut ::libc::c_int, maker: *mut ::libc::c_int,
                     overVolted: *mut ::libc::c_int) -> ();
    pub fn wpiPinToGpio(wpiPin: ::libc::c_int) -> ::libc::c_int;
    pub fn physPinToGpio(physPin: ::libc::c_int) -> ::libc::c_int;
    pub fn setPadDrive(group: ::libc::c_int, value: ::libc::c_int) -> ();
    pub fn getAlt(pin: ::libc::c_int) -> ::libc::c_int;
    pub fn pwmToneWrite(pin: ::libc::c_int, freq: ::libc::c_int) -> ();
    pub fn digitalWriteByte(value: ::libc::c_int) -> ();
    pub fn pwmSetMode(mode: ::libc::c_int) -> ();
    pub fn pwmSetRange(range: ::libc::c_uint) -> ();
    pub fn pwmSetClock(divisor: ::libc::c_int) -> ();
    pub fn gpioClockSet(pin: ::libc::c_int, freq: ::libc::c_int) -> ();
    pub fn waitForInterrupt(pin: ::libc::c_int, mS: ::libc::c_int)
     -> ::libc::c_int;
    pub fn wiringPiISR(pin: ::libc::c_int, mode: ::libc::c_int,
                       function: ::std::option::Option<extern "C" fn() -> ()>)
     -> ::libc::c_int;
    pub fn piThreadCreate(_fn:
                              ::std::option::Option<extern "C" fn(arg1:
                                                                      *mut ::libc::c_void)
                                                        ->
                                                            *mut ::libc::c_void>)
     -> ::libc::c_int;
    pub fn piLock(key: ::libc::c_int) -> ();
    pub fn piUnlock(key: ::libc::c_int) -> ();
    pub fn piHiPri(pri: ::libc::c_int) -> ::libc::c_int;
    pub fn delay(howLong: ::libc::c_uint) -> ();
    pub fn delayMicroseconds(howLong: ::libc::c_uint) -> ();
    pub fn millis() -> ::libc::c_uint;
    pub fn micros() -> ::libc::c_uint;
}
