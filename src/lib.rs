#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("App:\n  {source}"), visibility(pub))]
    App { source: app::Error },

    #[non_exhaustive]
    #[snafu(display("UI:\n  {source}"), visibility(pub))]
    Ui { source: ui::Error },

    #[non_exhaustive]
    #[snafu(display("Actions:\n  {source}"), visibility(pub))]
    Actions { source: actions::Error },
}

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

pub mod actions;
pub mod app;
pub mod data;
pub mod ui;

// endregion: MODULES

// region: RE-EXPORTS

pub use actions::*;
pub use app::*;
pub use data::*;
pub use ui::*;

// endregion: RE-EXPORTS
