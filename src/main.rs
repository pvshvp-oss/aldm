fn main() -> Result<(), Error> {
    Ok(
        Cli::run()?, // Dispatch commandline argument handling to its own distinct module
    )
}

// region: IMPORTS

use aldm::Cli;
use aldm::Error;

// endregion: IMPORTS
