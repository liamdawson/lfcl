#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate lfclib;

fn main() {
    let ctx = lfclib::device::LuxaforContext::new().unwrap();
    let devices = ctx.devices(lfclib::consts::device::FULL_FLAG).unwrap();

    for d in devices {
        match d.solid(255, 0, 255) {
            Ok(_) => {},
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
