pub struct Gui {}

impl RunApp for Gui {
    fn run_app() -> Result<Option<Box<dyn Any>>, crate::Error> {
        todo!();
    }
}

impl InitLog for Gui {}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    Dummy {},
}

// region: IMPORTS

use crate::app::logging::InitLog;
use crate::app::RunApp;
use snafu::Snafu;
use std::any::Any;

// endregion: IMPORTS
