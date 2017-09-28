#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate llclib;

fn main() {
    let ctx = llclib::device::LuxaforContext::new().unwrap();
    let devices = ctx.devices(llclib::consts::device::FULL_FLAG).unwrap();

    for d in devices {
        println!("a device");
    }
}
