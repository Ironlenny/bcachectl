use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use std::collections::HashMap;
use failure::ResultExt;
use exitfailure::ExitFailure;
use toml;
use serde_derive::Deserialize;

// Holds the per device configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    device: Vec<Device>,
}

// Represents a device's configuration
#[derive(Debug, Deserialize)]
pub struct Device {
    name: String,
    cache_mode: Option<String>,
    sequential_cutoff: Option<u32>,
    get_setting: Option<String>
}

// Struct for the `Commands` object
pub struct Commands {
    commands: HashMap<PathBuf, Option<String>>,
    args: Cli,
}

// Declare command-line arguments
#[derive(Debug, StructOpt)]
#[structopt(about = "A program to control Bcache devices", rename_all =
            "kebab-case", author = "Jacob Riddle")]
enum Cli {
    /// Set device settings
    #[structopt(name = "set")]
    Device {
        /// The name of the device to control. Ex. bcache0
        name: String,

        /// Set caching mode. Modes are writethrough, writeback, writearound,
        /// and none
        #[structopt(short, long)]
        cache_mode: Option<String>,

        #[structopt(short, long)]
        /// Set cutoff for sequential reads and writes
        sequential_cutoff: Option<u32>,
    },

    /// Get device settings
    #[structopt(name = "get")]
    Get {
        /// Name of device
        name: String,

        /// Get setting. Accepts cache_mode, and sequential_cutoff
        setting: String
    },

    /// Load a configuration file
    #[structopt(name = "load")]
    Config {
        /// Path to configuration file
        #[structopt(default_value = "/etc/bcache/bcache.conf",
                    parse(from_os_str))]
        path: PathBuf,
    },

    /// Suspend all caching devices. Use 'load' when resuming
    #[structopt(name = "suspend")]
    Suspend {
        /// Path to configuration file
        #[structopt(default_value = "/etc/bcache/bcache.conf",
                    parse(from_os_str))]
        path: PathBuf,
    },
}

// Methods for the `Commands` object
impl Commands {
    pub fn new () -> Self {
        Commands { commands: HashMap::new(), args: Cli::from_args() }
    }

    // Parse command-line arguments. Return a `Config` or an `ExitFailure`
    pub fn parse_args(& self) -> Result<Config, ExitFailure> {

        match &self.args {
            // Set device configuration
            Cli::Device {name, cache_mode, sequential_cutoff} => {

                if !cache_mode.is_some() && !sequential_cutoff.is_some() {
                    let error = Err(failure::err_msg("no valid setting given"));
                    return Ok(error.context("See 'bcachectl set' usage"
                                            .to_string())?);
                }

                let device = Device {name: name.to_string(), cache_mode:
                                     cache_mode.clone(), sequential_cutoff:
                                     sequential_cutoff.clone(),
                                     get_setting: None };

                return Ok(Config {device: vec![device]});
            },
            // Load configuration file
            Cli::Config {path} => {
                return self.parse_conf(path.to_path_buf());
            },
            // Set all device cache modes to 'none'
            Cli::Suspend {path} => {
                let mut config = self.parse_conf(path.to_path_buf())?;

                for dev in config.device.iter_mut() {
                    dev.cache_mode = Some(String::from("none"));
                }

                return Ok(config)
            },
            // Get device setting
            Cli::Get {name, setting} => {
                let value = match setting.as_str() {
                    "cache_mode" => Some(String::from("cache_mode")),
                    "sequential_cutoff" => Some(String::from(
                        "sequential_cutoff")),
                    _ => { let error = Err(failure::err_msg("no valid setting given"));
                           return Ok(error.context("See 'bcachectl get' usage"
                                                   .to_string())?); },
                };

                let device = Device {name: name.to_string(), cache_mode: None,
                                     sequential_cutoff: None,
                                     get_setting: value};

                return Ok(Config {device: vec![device]});
            }

        }
    }

    // Parse configuration file. Take a path to file and return a `Config` or an
    // `ExitFailure`
    fn parse_conf(&self, path: PathBuf) -> Result<Config, ExitFailure> {

        let contents = fs::read_to_string(&path).with_context(|_| format!(
            "could not read file {}", path.display()))?;
        let config: Config = toml::from_str(&contents).with_context(|_| format!(
            "could not parse file {}", path.display()))?;

        Ok(config)
    }

    // Takes the filename of the setting and the name of the caching device and
    // generates a path.
    fn gen_path(&self, file: &str, name: &str) -> PathBuf {
        let path = format!("/sys/block/{}/bcache/{}", name, file);
        return PathBuf::from(path);
    }

    // Takes a Config struct and iterates through generating commands. The
    // commands are then pushed to the `commands` hashmap.
    pub fn gen_commands(&mut self, config: Config) {
        for device in config.device.iter() {
            let name = &device.name;

            if let Some(mode) = &device.cache_mode {
                let path = self.gen_path("cache_mode", name);
                &mut self.commands.insert(path, Some(mode.to_string()));
            }

            if let Some(value) = &device.sequential_cutoff {
                let path = self.gen_path("sequential_cutoff", name);
                &mut self.commands.insert(path, Some(value.to_string()));
            }

            if let Some(value) = &device.get_setting {
                let path = self.gen_path(value, name);
                &mut self.commands.insert(path, None);
            }
        }
    }

    pub fn run_commands(&mut self) -> Result<Vec<String>, ExitFailure> {
        let mut output: Vec<String> = Vec::new();

        for (path, value) in & self.commands {

            if value.is_some() {
                fs::write(&path, value.clone().unwrap())
                    .with_context(|_| format!("Could not write to file {}",
                                              path.display()))?;
            }
            else {
                &output.push(format!("{}", fs::read_to_string(&path)
                                     .with_context(|_| format!(
                                         "Could not read file {}",
                                         path.display()))?));
            }
        }

        Ok(output)
    }
}
