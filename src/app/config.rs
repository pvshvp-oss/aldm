pub trait Config {}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    Dummy {},
}

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS
