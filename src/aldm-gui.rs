fn main() -> Result<(), Error> {
    Ok(
        _ = Gui::run_app()?, // Dispatch GUI handling to its own distinct module
    )
}

// region: IMPORTS

use aldm::Error;
use aldm::Gui;
use aldm::RunApp;

// endregion: IMPORTS
