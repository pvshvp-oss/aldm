fn main() -> process::ExitCode {
    let return_value = gui::run_gui();
    match return_value {
        Ok(_) => process::ExitCode::from(0),
        Err(err_value) => {
            anstream::eprintln!(
                "{} {err_value}",
                "[ERROR]"
                    .bold()
                    .red()
            );
            process::ExitCode::from(1)
        }
    }
}

// region: IMPORTS

use aldm::ui::gui;
use owo_colors::OwoColorize;
use std::process;

// endregion: IMPORTS
