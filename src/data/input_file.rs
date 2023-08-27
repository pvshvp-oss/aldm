#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""))]
    InputFileDummy {},
}

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS
