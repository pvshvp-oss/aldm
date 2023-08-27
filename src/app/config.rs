pub fn init_config() -> Result<Config, Error> {
    Ok(Config {})
}

#[derive(Clone, Debug, Serialize, Deserialize, Readable, Writable)]
pub struct Config {}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    ConfigDummy {},
}

// region: IMPORTS

use serde::{Deserialize, Serialize};
use snafu::Snafu;
use speedy::{Readable, Writable};

// endregion: IMPORTS
