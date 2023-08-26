lazy_static! {
    pub static ref APP_NAME: &'static str = "aldm";
}

pub trait RunApp {
    fn run_app() -> Result<Option<Box<dyn Any>>, crate::Error>;
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
}

// region: IMPORTS

use lazy_static::lazy_static;
use snafu::Snafu;
use std::any::Any;

// endregion: IMPORTS

// region: MODULES

pub mod config;
pub mod logging;

// endregion: MODULES

// region: RE-EXPORTS

pub use config::*;
pub use logging::*;
use tracing_appender::non_blocking::WorkerGuard;

// endregion: RE-EXPORTS
