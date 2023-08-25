pub struct Cli {}

impl RunApp for Cli {
    fn run_app() -> Result<Vec<WorkerGuard>, crate::Error> {
        let log_level_filter = clap_verbosity_flag::LevelFilter::Trace;
        let _worker_guards = Cli::init_log(log_level_filter.as_str().parse().ok())
            .context(LoggingSnafu {})
            .context(AppSnafu {})?;
        tracing::trace!("This is {}", "trace!".color(AnsiColors::Magenta));
        tracing::debug!("This is {}", "debug!".color(AnsiColors::Blue));
        tracing::info!("This is {}", "info!".color(AnsiColors::Green));
        tracing::warn!("This is {}", "warn!".color(AnsiColors::Yellow));
        tracing::error!("This is {}", "error!".color(AnsiColors::Red));
        Ok(_worker_guards)
    }
}

impl InitLog for Cli {}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    Dummy {},
}

// region: IMPORTS

use crate::{
    app::{logging::InitLog, LoggingSnafu, RunApp},
    AppSnafu,
};
use owo_colors::{AnsiColors, OwoColorize};
use snafu::{ResultExt, Snafu};
use tracing_appender::non_blocking::WorkerGuard;

// endregion: IMPORTS

// region: MODULES

mod cli_template {
    #[derive(clap::Parser, Debug)]
    pub struct CliTemplate {
        #[clap(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    }
}

//endregion: MODULES
