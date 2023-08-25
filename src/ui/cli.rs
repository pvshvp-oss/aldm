pub struct Cli {}

impl Launch for Cli {
    fn launch() -> Result<(), crate::Error> {
        Cli::init_log(None)
            .context(LoggingSnafu {})
            .context(AppSnafu {})?;
        tracing::info!("Hello!");
        Ok(())
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
    app::{logging::InitLog, Launch, LoggingSnafu},
    AppSnafu,
};
use snafu::{ResultExt, Snafu};

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
