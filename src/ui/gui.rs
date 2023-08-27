pub fn run_gui() -> Result<Option<Box<dyn Any>>, crate::Error> {
    config::init_config()
        .context(app::ConfigSnafu {})
        .context(crate::AppSnafu)?;

    let cli_input = CliTemplate::parse();
    let global_args = &cli_input.global_arguments;
    if global_args.is_uncolored() {
        anstream::ColorChoice::Never.write_global();
        owo_colors::set_override(false);
    }
    let mut handle = logging::init_log(global_args.verbosity_filter())
        .context(app::LoggingSnafu {})
        .context(crate::AppSnafu {})?;

    if global_args.is_json() {
        _ = handle
            .switch_to_json()
            .context(app::LoggingSnafu {})
            .context(crate::AppSnafu {})?;
    }

    tracing::info!("{:#?}", cli_input);
    tracing::trace!(
        "This is {}",
        "trace!".color(owo_colors::AnsiColors::Magenta)
    );
    tracing::debug!("This is {}", "debug!".color(owo_colors::AnsiColors::Blue));
    tracing::info!("This is {}", "info!".color(owo_colors::AnsiColors::Green));
    tracing::warn!("This is {}", "warn!".color(owo_colors::AnsiColors::Yellow));
    tracing::error!("This is {}", "error!".color(owo_colors::AnsiColors::Red));
    tracing::info!(target:"JSON", "This is JSON: {}", "{\"key\": \"value\"}");

    Ok(Some(Box::new(handle.worker_guards)))
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    GuiDummy {},
}

// region: IMPORTS

use crate::{
    app::{self, config, logging},
    ui::cli::CliModifier,
};
use clap::Parser;
use owo_colors::OwoColorize;
use snafu::{ResultExt, Snafu};
use std::any::Any;

// endregion: IMPORTS

// region: MODULES

mod gui_cli_template {
    #[derive(Parser, Debug)]
    #[command(version, author, about, args_conflicts_with_subcommands = true)]
    pub struct CliTemplate {
        #[clap(flatten)]
        pub global_arguments: cli::GlobalArgs,
    }

    // region: IMPORTS

    use crate::ui::cli;
    use clap::Parser;

    // endregion: IMPORTS
}

// endregion: MODULES

// region: RE-EXPORTS

pub use gui_cli_template::*;

// endregion: RE-EXPORTS
