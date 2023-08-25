fn main() -> Result<(), Error> {
    Ok(
        _ = Cli::run_app()?, // Dispatch commandline argument handling to its own distinct module
    )
}

// region: IMPORTS

use aldm::Cli;
use aldm::Error;
use aldm::RunApp;

// endregion: IMPORTS
