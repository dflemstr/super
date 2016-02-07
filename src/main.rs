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
mod error;
mod id;
mod supervisor;

use std::path;

fn main() {
    let matches = parse_cli_args();
    // .unwrap() is safe since CONFIG is required
    let config_path = matches.value_of("CONFIG").unwrap();

    run(&config_path).unwrap();
}

fn run<P>(config_path: P) -> error::Result<()> where P: AsRef<path::Path> {
    let config = try!(config::Config::read(config_path));
    let mut programs: Vec<_> =
        config.programs.iter().map(|(k, v)| (k.as_str(), v)).collect();
    programs.sort_by_key(|&(_, ref p)| p.priority.unwrap_or(0));

    let supervisors = create_supervisors(&programs);

    for supervisor in supervisors.iter() {
        supervisor.start();
    }

    for supervisor in supervisors.iter().rev() {
        supervisor.stop();
    }

    Ok(())
}

fn create_supervisors(programs: &[(&str, &config::Program)]) -> Vec<supervisor::Supervisor> {
    let mut supervisors = Vec::with_capacity(programs.len());

    for &(key, program) in programs.iter() {
        let num_procs = program.num_procs.unwrap_or(1);
        for i in 1..num_procs {
            let id = id::Id::new(
                key.to_owned(),
                if num_procs == 1 { None } else { Some(i) });

            let supervisor =
                supervisor::Supervisor::new(id, program.clone());
            supervisors.push(supervisor);
        }
    }

    supervisors
}

fn parse_cli_args<'n, 'a>() -> clap::ArgMatches<'n, 'a> {
    clap_app!(super =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: "David Flemström (dflemstr)")
        (about: "A supervision process optimized for use within Docker containers")
        (@arg CONFIG: -c --config +required +takes_value "Config file to use")
    ).get_matches()
}
