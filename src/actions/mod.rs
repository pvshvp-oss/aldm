#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("Could not list:\n  {source}"))]
    CouldNotList {source: list::Error},

    #[non_exhaustive]
    #[snafu(display("Could not search:\n  {source}"))]
    CouldNotSearch {source: search::Error},

    #[non_exhaustive]
    #[snafu(display("Could not install:\n  {source}"))]
    CouldNotInstall {source: install::Error},

    #[non_exhaustive]
    #[snafu(display("Could not generate database:\n  {source}"))]
    CouldNotGenerateDatabase {source: generate_db::Error},
}

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

pub mod list;
pub mod search;
pub mod install;
pub mod generate_db;

// endregion: MODULES

// region: RE-EXPORTS

pub use list::*;
pub use search::*;
pub use install::*;
pub use generate_db::*;

// endregion: RE-EXPORTS