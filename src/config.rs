use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Config {
    pub database_url: String,
    pub database_pool_size: u32,
}

impl Default for Config {
    fn default() -> Self {
        cfg_if! {
            if #[cfg(feature = "dev")] {
                let database_url = "database.db".to_owned();
            } else {
                // TODO: Change this
                let database_url = "database.db".to_owned();
            }
        }

        let database_pool_size = {
            let cpus = num_cpus::get();
            if cpus >= 2 {
                cpus as u32
            } else {
                2
            }
        };

        Self {
            database_url,
            database_pool_size,
        }
    }
}
