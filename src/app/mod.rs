pub trait Launch {
    fn launch() -> Result<(), crate::Error>;
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("Logging:\n  {source}"))]
    Logging {source: logging::Error},

    #[non_exhaustive]
    #[snafu(display("Config:\n  {source}"))]
    Config {source: config::Error},
}

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

pub mod config;
pub mod logging;

// endregion: MODULES

// region: RE-EXPORTS

pub use config::*;
pub use logging::*;

// endregion: RE-EXPORTS