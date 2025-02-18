use dotenv::dotenv;
use serde_derive::Deserialize;

fn default_connection_timeout_milisecs() -> u64 {
    500
}

fn default_user_agent() -> String {
    // The most popular user agent. See https://bitnodes.io/nodes/
    "/Satoshi:25.0.0/".to_string()
}

fn default_start_height() -> i32 {
    0
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_connection_timeout_milisecs")]
    pub connection_timeout_millisec: u64,
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
    #[serde(default = "default_start_height")]
    pub start_height: i32,
}

pub fn build() -> Config {
    dotenv().ok();
    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Error reading config values {:#?}", error),
    }
}
