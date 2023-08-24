lazy_static! {
    pub static ref APP_NAME: &'static str = "aldm";
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("App:\n  {source}"))]
    App {source: app::Error},

    #[non_exhaustive]
    #[snafu(display("UI:\n  {source}"))]
    Ui {source: ui::Error},

    #[non_exhaustive]
    #[snafu(display("Actions:\n  {source}"))]
    Actions {source: actions::Error},
}

// region: IMPORTS

use lazy_static::lazy_static;
use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

pub mod app;
pub mod ui;
pub mod data;
pub mod actions;

// endregion: MODULES

// region: RE-EXPORTS

pub use app::*;
pub use ui::*; 
pub use data::*;
pub use actions::*;

// endregion: RE-EXPORTS
