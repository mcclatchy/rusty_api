pub mod schema;
pub mod models;
pub mod graphql;

use serde::Deserialize;
use std::fs;

#[macro_use]
extern crate diesel;

#[derive(Deserialize)]
pub struct Config {
    pub(crate) api_key: String,
    pub(crate) database_url: String,
    pub(crate) debug: bool
}

impl Config {
    // TODO: learn how to turn this into a memory constant so the file isn't read each time
    fn get_configuration() -> Config {

        // open file
        let file = fs::read_to_string("settings.toml")
            .expect("Something went wrong reading settings.toml");

        // read file contents as config
        let config: Config = toml::from_str(file.as_str())
            .expect("Something went wrong processing settings string to config");

        // set the values
        Config {
            api_key: config.api_key,
            database_url: config.database_url,
            debug: config.debug,
        }

    }
}