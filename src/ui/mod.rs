#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("CLI:\n  {source}"))]
    Cli {source: cli::Error},

    #[non_exhaustive]
    #[snafu(display("GUI:\n  {source}"))]
    Gui {source: gui::Error},
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