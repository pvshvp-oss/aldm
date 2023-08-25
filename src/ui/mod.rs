#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("CLI:\n  {source}"), visibility(pub))]
    Cli { source: cli::Error },

    #[non_exhaustive]
    #[snafu(display("GUI:\n  {source}"), visibility(pub))]
    Gui { source: gui::Error },
}

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

pub mod cli;
pub mod gui;

// endregion: MODULES

// region: RE-EXPORTS

pub use cli::*;
pub use gui::*;

// endregion: RE-EXPORTS
