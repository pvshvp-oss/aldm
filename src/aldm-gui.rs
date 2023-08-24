fn main() -> Result<(), Error> {
    Ok(
        Gui::launch()?, // Dispatch GUI handling to its own distinct module
    )
}

// region: IMPORTS

use aldm::Gui;
use aldm::Launch;
use aldm::Error;

// endregion: IMPORTS