pub struct Gui {}

pub fn run_gui() -> Result<Option<Box<dyn Any>>, crate::Error> {
    todo!();
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    GuiDummy {},
}

// region: IMPORTS

use snafu::Snafu;
use std::any::Any;

// endregion: IMPORTS
