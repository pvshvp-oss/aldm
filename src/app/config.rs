pub trait Config {}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    ConfigDummy {},
}

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS
