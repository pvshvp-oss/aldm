#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("in the CLI: {source}"), visibility(pub))]
    Cli {
        #[snafu(backtrace)]
        source: cli::Error,
    },

    #[non_exhaustive]
    #[snafu(display("in the GUI: {source}"), visibility(pub))]
    Gui {
        #[snafu(backtrace)]
        source: gui::Error,
    },
}

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

pub mod cli;
pub mod gui;

// endregion: MODULES
