fn main() -> ExitCode {
    let return_value = Gui::run_app(); // Dispatch graphical interface handling to its own distinct module
    match return_value {
        Ok(_ok_value) => ExitCode::from(0),
        Err(err_value) => {
            eprintln!("{} {err_value}", "[ERROR]".bold().red());
            ExitCode::from(1)
        }
    }
}

// region: IMPORTS

use aldm::Gui;
use aldm::RunApp;
use anstream::eprintln;
use owo_colors::OwoColorize;
use std::process::ExitCode;

// endregion: IMPORTS
