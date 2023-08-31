#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""))]
    DatabaseDummy {},
}

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS
