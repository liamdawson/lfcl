#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate clap;
extern crate lfclib;

use clap::Arg;

fn main() {
    let matches : clap::ArgMatches = app_from_crate!()
        .arg(Arg::with_name("COLOUR")
            .help("Colour to set Luxafor flag to")
            .required(true)
            .index(1)).get_matches();

    let colour : [u8;3] = match matches
        .value_of("COLOUR")
        .map(|v| v.to_lowercase()) {

        Some(val) => {
            match val.as_str() {
                "r" => [255, 0, 0],
                "o" => [255, 64, 0],
                "y" => [255, 255, 0],
                "g" => [0, 255, 0],
                "b" => [0, 0, 255],
                "i" => [128, 0, 255],
                "v" => [255, 0, 255],
                _ => [0, 0, 0]
            }
        },
        _ => [0u8, 0u8, 0u8]
    };

    let ctx = lfclib::device::LuxaforContext::new().unwrap();
    let devices = ctx.devices(lfclib::consts::device::FULL_FLAG).unwrap();

    for d in devices {
        match d.solid(colour[0], colour[1], colour[2]) {
            Ok(_) => {},
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
