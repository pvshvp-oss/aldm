#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("Database:\n  {source}"))]
    Logging {source: database::Error},

    #[non_exhaustive]
    #[snafu(display("Input File:\n  {source}"))]
    Config {source: input_file::Error},
}

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

pub mod input_file;
pub mod database;

// endregion: MODULES

// region: RE-EXPORTS

pub use input_file::*;
pub use database::*;

// endregion: RE-EXPORTS