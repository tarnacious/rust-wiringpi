use std::process::Command;
use std::env;

fn main() {
    if !cfg!(feature = "noclib") {
        let out_dir = env::var("OUT_DIR").unwrap();
        match Command::new("make").arg("-e").arg("all").status() {
            Ok(status) if !status.success() => panic!("failed to build wiringPi C library (exit code {:?})", status.code()),
            Err(e) => panic!("failed to build wiringPi C library: {}", e),
            _ => {}
        }
        println!("cargo:rustc-flags=-L native={} -l static=wiringpi", out_dir);
        println!("cargo:rustc-flags=-L native={} -l static=wiringPiDev", out_dir);
    }
}
