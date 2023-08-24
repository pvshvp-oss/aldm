/// Captures the error contexts at the crate level
#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    CouldNotList {},

    #[non_exhaustive]
    CouldNotSearch {},

    #[non_exhaustive]
    CouldNotInstall {},

    #[non_exhaustive]
    CouldNotGenerateDatabase {},
}

// region: IMPORTS

use std::path::PathBuf;
use lazy_static::lazy_static;
use snafu::Snafu;
use dirs::home_dir;

// endregion: IMPORTS

// region: MODULES

/// For organization/grouping of commandline-related concerns
mod cli;

// endregion: MODULES

// region: RE-EXPORTS

pub use cli::Cli;

// endregion: RE-EXPORTS
