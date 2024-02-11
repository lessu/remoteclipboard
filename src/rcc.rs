use std::{fs::File, io::Read};

use clap::{Command, Arg, arg, value_parser};
use toml::value::Table;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
//    ip: String,
//    port: Option<u16>,
    version:u16
}

fn main() {
    let home_path = env!("HOME");
    let default_profile_path = [home_path,"/.rclip_profile"].join("");
    let matches = Command::new("rcc")
        .version("1.0")
        .author("lessu")
        .about("remote-clipbord-copy")
        .args(&[
            arg!(profile: -p --profile <profile>)
                .value_parser(value_parser!(String))
        ])
        .get_matches();

}