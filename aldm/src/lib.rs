pub fn run_common<C>() -> Result<(C, Vec<WorkerGuard>), crate::Error>
where
    C: clap::Parser + CliModifier + fmt::Debug,
    <C as GlobalArguments>::L: LogLevel,
{
    // Obtain CLI arguments
    let cli_input = C::parse();

    // Initialize XDG App directories
    let xdg_app_dirs = xdg::BaseDirectories::with_prefix(*app::APP_NAME)
        .context(app::FetchXdgAppDirectoriesSnafu {})
        .context(AppSnafu {})?;

    // Load Configuration
    // Obtain configuration from the environment, and then layer it with
    // configuation from files in the order of priority, followed by
    // ensuring that a default configuration is loaded if none of the above
    // have valid fields set.
    let optional_preferred_config_filepath = cli_input
        .config_file()
        .clone();
    let xdg_config_filepath = xdg_app_dirs.get_config_file(format!("{}.yaml", *app::APP_NAME));
    let global_config_filepath: PathBuf = format!("/etc/{}.yaml", *app::APP_NAME).into();
    let mut config = config::Config::new();
    config
        .env()
        .optional_filepath(optional_preferred_config_filepath.clone())
        .context(ConfigSnafu {})
        .context(AppSnafu {})?
        .filepath(xdg_config_filepath.clone())
        .context(ConfigSnafu {})
        .context(AppSnafu {})?
        .filepath(global_config_filepath.clone())
        .context(ConfigSnafu {})
        .context(AppSnafu {})?
        .ensure_loaded();
    // Store readable config filepaths that are likely used
    let config_filepaths: Vec<PathBuf> = vec![
        optional_preferred_config_filepath,
        Some(xdg_config_filepath.clone()),
        Some(global_config_filepath.clone()),
    ]
    .into_iter()
    .all_readable_paths()
    .collect();

    // // Begin logging with preferred log directory and preferred verbosity
    // let config_verbosity_filter: Option<LevelFilter> = (&config)
    //     .log_level_filter
    //     .and_then(|lf| {
    //         lf.as_str()
    //             .parse()
    //             .ok()
    //     });
    // let verbosity_filter = cli_input
    //     .verbosity_filter()
    //     .or(config_verbosity_filter);
    // let (mut handle, log_filepath) = logging::init_log(config.log_dirpath, verbosity_filter)
    //     .context(app::LoggingSnafu {})
    //     .context(crate::AppSnafu {})?;

    // // Configure Special Logging Formats
    // // Turn off colors if needed
    // if cli_input.is_uncolored()
    //     || config
    //         .no_color
    //         .unwrap_or(false)
    // {
    //     anstream::ColorChoice::Never.write_global();
    //     owo_colors::set_override(false);
    // }
    // // Modify output format if Plain or Json output is desired
    // if cli_input.is_json() {
    //     _ = handle
    //         .switch_to_json()
    //         .context(app::LoggingSnafu {})
    //         .context(crate::AppSnafu {})?;
    // } else if cli_input.is_plain() {
    //     _ = handle
    //         .switch_to_plain()
    //         .context(app::LoggingSnafu {})
    //         .context(crate::AppSnafu {})?;
    // } else if cli_input.is_test() {
    //     _ = handle
    //         .switch_to_test()
    //         .context(app::LoggingSnafu {})
    //         .context(crate::AppSnafu {})?;
    // }

    // // Welcome message
    // tracing::debug!(
    //     "{} - {}",
    //     "ALDM".bold(),
    //     "A Driver Manager for Arch Linux".magenta()
    // );
    // tracing::debug!(
    //     "{}  {} {}",
    //     console::Emoji("✉️", ""),
    //     "shiva.patt".italic(),
    //     "<shiva.patt.oss@gmail.com, shivanandvp@rebornos.org>".italic()
    // );
    // tracing::debug!(
    //     target:"TEST", "{}{}{}{}{}{}{}{}",
    //     "███".black(),
    //     "███".red(),
    //     "███".green(),
    //     "███".yellow(),
    //     "███".blue(),
    //     "███".purple(),
    //     "███".cyan(),
    //     "███".white()
    // );
    // tracing::debug!(
    //     target:"TEST", "{}{}{}{}{}{}{}{}",
    //     "███".bright_black(),
    //     "███".bright_red(),
    //     "███".bright_green(),
    //     "███".bright_yellow(),
    //     "███".bright_blue(),
    //     "███".bright_purple(),
    //     "███".bright_cyan(),
    //     "███".bright_white()
    // );

    // // Test messages
    // tracing::trace!(target:"TEST", "{} Testing trace!...", console::Emoji("🧪", ""));
    // tracing::debug!(target:"TEST", "{} Testing debug!...", console::Emoji("🧪", ""));
    // tracing::info!(target:"TEST", "{} Testing info!...", console::Emoji("🧪", ""));
    // tracing::warn!(target:"TEST", "{} Testing warn!...", console::Emoji("🧪", ""));
    // tracing::error!(target:"TEST", "{} Testing error!...", console::Emoji("🧪", ""));

    // tracing::info!(target:"JSON", "{} Testing: {}", console::Emoji("🧪", ""), "{\"JSON\": \"Target\"}");
    // tracing::info!(target:"PLAIN", "{} Testing: Plain Target", console::Emoji("🧪", ""));

    // tracing::debug!(
    //     "{}  The {} is {}... {}",
    //     console::Emoji("⚙️", ""),
    //     "configuration".cyan(),
    //     "loaded".green(),
    //     console::Emoji("✅", ""),
    // );
    // tracing::debug!(
    //     "{} The {} has {}... {}",
    //     console::Emoji("📝", ""),
    //     "logging".cyan(),
    //     "begun".green(),
    //     console::Emoji("✅", ""),
    // );

    // tracing::debug!(
    //     "{} {} {:?}",
    //     console::Emoji("📂", ""),
    //     "Config Filepath(s):".magenta(),
    //     config_filepaths,
    // );
    // tracing::debug!(
    //     "{} {} {:?}",
    //     console::Emoji("📂", ""),
    //     "Log Filepath:".magenta(),
    //     log_filepath
    // );

    // tracing::trace!(
    //     "{}  {} {:#?}",
    //     console::Emoji("⌨️", ""),
    //     "CLI input arguments:"
    //         .magenta()
    //         .dimmed(),
    //     cli_input.dimmed()
    // );

    // Ok((cli_input, handle.worker_guards))
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("in the application: {source}"), visibility(pub))]
    App {
        #[snafu(backtrace)]
        source: app::Error,
    },

    #[non_exhaustive]
    #[snafu(display("in the UI: {source}"), visibility(pub))]
    Ui {
        #[snafu(backtrace)]
        source: ui::Error,
    },

    #[non_exhaustive]
    #[snafu(display("in an action:{source}"), visibility(pub))]
    Actions {
        #[snafu(backtrace)]
        source: actions::Error,
    },
}

// region: IMPORTS

use app::{Permissions, ValidPaths};
use clap_verbosity_flag::LogLevel;
use core::fmt;
use owo_colors::OwoColorize;
use snafu::{ResultExt, Snafu};
use std::{any, path::PathBuf};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::filter::LevelFilter;
use ui::{CliModifier, GlobalArguments};

use crate::app::{
    config::{self, Configuration},
    ConfigSnafu,
};

// endregion: IMPORTS

// region: MODULES

pub mod actions;
pub mod app;
pub mod data;
pub mod ui;

// endregion: MODULES
