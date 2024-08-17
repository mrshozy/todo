pub extern crate env_config;
pub use env_config::*;
use getset::{
    CopyGetters,
    Getters,
};

#[derive(EnvConfig, Debug, Clone, Getters, CopyGetters)]
pub struct Config {
    #[getset(get = "pub with_prefix")]
    #[env(rename = "DATABASE_URL")]
    database_url: String,
    #[getset(get_copy = "pub with_prefix")]
    #[env(default = 3000)]
    #[env(rename = "PORT")]
    port: u16,
}
