#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate clap;
extern crate serde;
#[cfg(test)]
extern crate serde_json;
extern crate time;
extern crate toml;

mod config;
mod id;
mod supervisor;
mod supervisor_range;

use std::fs;
use std::path;

fn main() {
    let matches = parse_cli_args();

    let config_path = matches.value_of("CONFIG").unwrap();
    let mut programs: Vec<_> =
        read_config(&config_path).programs.into_iter().collect();
    programs.sort_by_key(|&(_, ref a)| a.priority.unwrap_or(0));
    let supervisors: Vec<_> =
        programs.into_iter().flat_map(create_supervisor).collect();

    for supervisor in supervisors.iter() {
        supervisor.start();
    }

    for supervisor in supervisors.iter().rev() {
        supervisor.stop();
    }
}

fn create_supervisor(elem: (String, config::Program))
                     -> supervisor_range::SupervisorRange {
    let (id, program) = elem;
    let n = program.num_procs.unwrap_or(1);
    supervisor_range::SupervisorRange::new(id, program, n)
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
