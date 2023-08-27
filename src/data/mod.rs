#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""))]
    DataDummy {},
}

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

pub mod database;
pub mod input_file;

// endregion: MODULES

// region: RE-EXPORTS

pub use database::*;
pub use input_file::*;

// endregion: RE-EXPORTS
