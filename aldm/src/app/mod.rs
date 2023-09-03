lazy_static! {
    pub static ref APP_NAME: &'static str = "aldm";
}

pub trait OptionalPathListPermissions<P>: Iterator<Item = Option<P>> + Sized
where
    P: Into<PathBuf>,
{
    fn first_readable_path(mut self) -> Option<P> {
        self.find(|p| {
            p.map(|p| permissions::is_readable(Into::<PathBuf>::into(p)).unwrap_or(false))
                .unwrap_or(false)
        })
        .flatten()
    }

    fn first_writable_path(mut self) -> Option<P> {
        self.find(|p| {
            p.map(|p| permissions::is_writable(Into::<PathBuf>::into(p)).unwrap_or(false))
                .unwrap_or(false)
        })
        .flatten()
    }

    fn all_readable_paths(self) -> Box<dyn Iterator<Item = P>> {
        Box::new(
            self.filter(|p: &Option<P>| {
                p.map(|p| permissions::is_readable(Into::<PathBuf>::into(p)).unwrap_or(false))
                    .unwrap_or(false)
            })
            .flat_map(std::convert::identity),
        )
    }

    fn all_writable_paths(self) -> Box<dyn Iterator<Item = P>> {
        Box::new(
            self.filter(|p: &Option<P>| {
                p.map(|p| permissions::is_writable(Into::<PathBuf>::into(p)).unwrap_or(false))
                    .unwrap_or(false)
            })
            .flat_map(std::convert::identity),
        )
    }
}

impl<T, P> OptionalPathListPermissions<P> for T
where
    T: Iterator<Item = Option<P>>,
    P: Into<PathBuf>,
{
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

    #[non_exhaustive]
    #[snafu(
        display("could not retreive the XDG app directories for the user: {source}"),
        visibility(pub)
    )]
    FetchXdgAppDirectories { source: xdg::BaseDirectoriesError },
}

// region: IMPORTS

use lazy_static::lazy_static;
use snafu::Snafu;
use std::{
    convert,
    iter::{self, FlatMap},
    path::{Path, PathBuf},
};

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
