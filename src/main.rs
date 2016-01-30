#[macro_use]
extern crate clap;
extern crate serde;
#[cfg(test)]
extern crate serde_json;
extern crate time;
extern crate toml;

mod config;
mod supervisor;

use std::fs;
use std::path;

fn main() {
    let matches = parse_cli_args();

    let config_path = matches.value_of("CONFIG").unwrap();
    let programs = read_config(&config_path).programs;
    let supervisors = programs.into_iter()
        .map(|(id, p)| supervisor::Supervisor::start(id, p));

    loop {}
}

fn read_config<P>(path: P) -> config::Config where P: AsRef<path::Path> {
    use std::io::Read;
    use std::str::FromStr;

    let mut file = fs::File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut d = toml::Decoder::new(toml::Value::from_str(&data).unwrap());
    serde::de::Deserialize::deserialize(&mut d).unwrap()
}

fn parse_cli_args<'n, 'a>() -> clap::ArgMatches<'n, 'a> {
    clap_app!(super =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: "David Flemström (dflemstr)")
        (about: "A supervision process optimized for use within Docker containers")
        (@arg CONFIG: -c --config +required +takes_value "Config file to use")
    ).get_matches()
}
