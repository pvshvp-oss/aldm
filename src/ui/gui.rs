pub struct Gui {}

impl Launch for Gui {
    fn launch() -> Result<(), crate::Error> {
        todo!();
    }
}

impl InitLog for Gui {}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""))]
    Dummy {},
}

// region: IMPORTS

use crate::app::Launch;
use crate::app::logging::InitLog;
use snafu::Snafu;

// endregion: IMPORTS
