use std::collections;
use std::path;

use serde;

use time;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub programs: collections::HashMap<String, Program>,
}

// Create a new type to allow for better config file syntax
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Duration(time::Duration);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Program {
    // Roughly inspired by supervisord documentation

    /// The command to execute in order to start this program.
    pub command: Vec<String>,

    /// The number of instances of this program that should be
    /// running.
    #[serde(default)]
    pub num_procs: Option<u32>,

    /// The relative priority of the program in the start and shutdown
    /// ordering.  Lower priorities indicate programs that start first
    /// and shut down last.
    #[serde(default)]
    pub priority: Option<i32>,

    /// How long the program must be alive for it to be considered
    /// "started"
    #[serde(default)]
    pub start_time: Option<Duration>,

    /// How many times the program will try to be started before
    /// giving up.
    #[serde(default)]
    pub start_retries: Option<u32>,

    /// The strategy used for restarting the program when it exits.
    #[serde(default)]
    pub auto_restart: Option<AutoRestart>,

    /// The set of "expected" exit codes when evaluating the
    /// auto_restart condition.
    #[serde(default)]
    pub exit_codes: collections::HashSet<i32>,

    /// The signal used to stop the program.
    #[serde(default)]
    pub stop_signal: Option<i32>,

    /// The amount of time to wait for a SIGCHLD after sending
    /// stop_signal; after the time expires, SIGKILL will be sent.
    #[serde(default)]
    pub stop_time: Option<Duration>,

    /// Send the stop_signal to all processes spawned by this program
    /// (including child processes).
    #[serde(default)]
    pub stop_as_group: bool,

    /// Send the kill_signal to all processes spawned by this program
    /// (including child processes).
    #[serde(default)]
    pub kill_as_group: bool,

    /// Additional environment variables to set, that will override
    /// those environment variables that are in the environment of the
    /// supervisor.
    #[serde(default)]
    pub environment: collections::HashMap<String, String>,

    /// The working directory of the program.
    #[serde(default)]
    pub directory: Option<path::PathBuf>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AutoRestart {
    /// Always restart the program if it exits
    Always,
    /// Only restart the program if its exit code was not in
    /// program.exit_codes.
    OnUnexpected,
    /// Only restart the program if its exit code was in
    /// program.exit_codes.
    OnExpected,
    /// Never restart the program.
    Never,
}

impl serde::Deserialize for Duration {
    fn deserialize<D>(deserializer: &mut D) -> Result<Duration, D::Error>
        where D: serde::Deserializer {

        // TODO: maybe don't use a function-local struct...
        struct DurationVisitor;

        impl serde::de::Visitor for DurationVisitor {

            type Value = Duration;

            fn visit_str<E>(&mut self, value: &str) -> Result<Duration, E>
                where E: serde::de::Error {

                let mut result = time::Duration::zero();
                let mut compound = 0;
                for c in value.chars() {
                    match c {
                        '0'...'9' =>
                            compound = compound * 10 + (c as i64 - '0' as i64),
                        'w' => {
                            result = result + time::Duration::weeks(compound);
                            compound = 0;
                        },
                        'd' => {
                            result = result + time::Duration::days(compound);
                            compound = 0;
                        },
                        'h' => {
                            result = result + time::Duration::hours(compound);
                            compound = 0;
                        },
                        'm' => {
                            result = result + time::Duration::minutes(compound);
                            compound = 0;
                        },
                        's' => {
                            result = result + time::Duration::seconds(compound);
                            compound = 0;
                        },

                        _ => return Err(
                            serde::de::Error::syntax(
                                &format!("Unexpected character {:?}", c))),
                    }
                }

                Ok(Duration(result))
            }
        }

        deserializer.visit(DurationVisitor)
    }
}

impl serde::Serialize for Duration {

    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer {
        // Some accuracy is lost here if the duration is more exact
        // than seconds, but meh
        serializer.visit_str(&format!("{}s", self.0.num_seconds()))
    }
}

#[cfg(test)]
mod test {
    use serde;
    use serde_json;
    use time;
    use toml;

    // Encodes a Serializable thing to JSON
    macro_rules! encode( ($e:expr) => ({
        serde_json::to_string($e)
    }) );

    // Decodes a Serializable thing from JSON
    macro_rules! decode( ($e:expr) => ({
        serde_json::from_str($e)
    }) );

    #[test]
    fn config_decode() {
        let config = r#"
[programs.echo]
command = ["echo", "hello"]
num_procs = 3
priority = 6
start_time = "1m30s"
stop_as_group = true

[programs.foo]
command = ["foo"]
"#;

        let mut d = toml::Decoder::new(config.parse().unwrap());
        let actual: super::Config =
            serde::de::Deserialize::deserialize(&mut d).unwrap();
        assert_eq!("echo", &actual.programs["echo"].command[0]);
        assert_eq!("hello", &actual.programs["echo"].command[1]);
        assert_eq!(Some(3), actual.programs["echo"].num_procs);
        assert_eq!(Some(6), actual.programs["echo"].priority);
        assert_eq!(Some(super::Duration(time::Duration::seconds(90))),
                   actual.programs["echo"].start_time);
        assert_eq!(true, actual.programs["echo"].stop_as_group);
        assert_eq!("foo", actual.programs["foo"].command[0]);
    }

    #[test]
    fn duration_decode_weeks() {
        let expected = super::Duration(time::Duration::weeks(3));
        let actual = decode!("\"3w\"").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration_decode_days() {
        let expected = super::Duration(time::Duration::days(3));
        let actual = decode!("\"3d\"").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration_decode_hours() {
        let expected = super::Duration(time::Duration::hours(3));
        let actual = decode!("\"3h\"").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration_decode_minutes() {
        let expected = super::Duration(time::Duration::minutes(3));
        let actual = decode!("\"3m\"").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration_decode_seconds() {
        let expected = super::Duration(time::Duration::seconds(3));
        let actual = decode!("\"3s\"").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration_decode_multi_digit() {
        let expected = super::Duration(time::Duration::seconds(123456789));
        let actual = decode!("\"123456789s\"").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration_decode_multi_unit() {
        let expected = super::Duration(time::Duration::seconds(788645));
        let actual = decode!("\"1w2d3h4m5s\"").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration_decode_multi_digit_unit() {
        let expected = super::Duration(time::Duration::seconds(10401570));
        let actual = decode!("\"12w34d56h78m90s\"").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration_encode() {
        let expected = "\"10401570s\"";
        let actual = encode!(
            &super::Duration(time::Duration::seconds(10401570))).unwrap();
        assert_eq!(expected, actual);
    }
}
