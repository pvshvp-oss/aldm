fn main() -> Result<(), Error> {
    Ok(
        Cli::launch()?, // Dispatch commandline argument handling to its own distinct module
    )
}

// region: IMPORTS

use aldm::Cli;
use aldm::Launch;
use aldm::Error;

// endregion: IMPORTS
