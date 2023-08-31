lazy_static! {
    pub static ref APP_NAME: &'static str = "aldm";
}

pub fn first_readable_valid_path<'a>(
    paths: &'a Vec<impl AsRef<Path> + 'a>,
) -> Option<impl AsRef<Path> + 'a> {
    paths
        .iter()
        .find(|p| permissions::is_readable(p).unwrap_or(false))
}

pub fn first_writable_valid_path<'a>(
    paths: &'a Vec<impl AsRef<Path> + 'a>,
) -> Option<impl AsRef<Path> + 'a> {
    paths
        .iter()
        .find(|p| permissions::is_writable(p).unwrap_or(false))
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("in logging: {source}"), visibility(pub))]
    Logging {
        #[snafu(backtrace)]
        source: logging::Error,
    },

    #[non_exhaustive]
    #[snafu(display("in configuration: {source}"), visibility(pub))]
    Config {
        #[snafu(backtrace)]
        source: config::Error,
    },

    #[non_exhaustive]
    #[snafu(display("in internationalization: {source}"), visibility(pub))]
    Internationalization {
        #[snafu(backtrace)]
        source: i18n::Error,
    },
}

// region: IMPORTS

use lazy_static::lazy_static;
use snafu::Snafu;
use std::path::Path;

// endregion: IMPORTS

// region: MODULES

pub mod config;
pub mod i18n;
pub mod logging;

// endregion: MODULES

// region: RE-EXPORTS

pub use config::*;
pub use i18n::*;
pub use logging::*;

// endregion: RE-EXPORTS
