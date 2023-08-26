#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("in the application: {source}"), visibility(pub))]
    App {
        #[snafu(backtrace)]
        source: app::Error,
    },

    #[non_exhaustive]
    #[snafu(display("in the UI: {source}"), visibility(pub))]
    Ui {
        #[snafu(backtrace)]
        source: ui::Error,
    },

    #[non_exhaustive]
    #[snafu(display("in an action:{source}"), visibility(pub))]
    Actions {
        #[snafu(backtrace)]
        source: actions::Error,
    },
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
