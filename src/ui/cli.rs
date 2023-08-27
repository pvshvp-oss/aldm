pub struct Cli {}

impl RunApp for Cli {
    fn run_app() -> Result<Option<Box<dyn Any>>, crate::Error> {
        let cli_input = CliTemplate::parse();

        if cli_input.is_uncolored() {
            anstream::ColorChoice::Never.write_global();
            owo_colors::set_override(false);
        }

        let mut handle = Cli::init_log(cli_input.verbosity_filter())
            .context(LoggingSnafu {})
            .context(AppSnafu {})?;

        if cli_input.is_json() {
            _ = handle
                .switch_to_json()
                .context(LoggingSnafu {})
                .context(AppSnafu {})?;
        }

        tracing::info!("{:#?}", cli_input);
        tracing::trace!("This is {}", "trace!".color(AnsiColors::Magenta));
        tracing::debug!("This is {}", "debug!".color(AnsiColors::Blue));
        tracing::info!("This is {}", "info!".color(AnsiColors::Green));
        tracing::warn!("This is {}", "warn!".color(AnsiColors::Yellow));
        tracing::error!("This is {}", "error!".color(AnsiColors::Red));
        let a = "{}";
        tracing::info!(target:"JSON", "This is JSON: {}", a);

        Ok(Some(Box::new(handle.worker_guards)))
    }
}

impl InitLog for Cli {}

pub trait CliModifier {
    fn verbosity_filter(&self) -> Option<LevelFilter>;
    fn is_uncolored(&self) -> bool;
    fn is_colored(&self) -> bool {
        !self.is_uncolored()
    }
    fn is_json(&self) -> bool;
}

impl CliModifier for CliTemplate {
    fn verbosity_filter(&self) -> Option<LevelFilter> {
        if self
            .global_arguments
            .plain_flag
            || self
                .global_arguments
                .json_flag
        {
            return Some(LevelFilter::INFO);
        }

        let verbosity_flag_filter = self
            .global_arguments
            .verbose
            .log_level_filter();

        if verbosity_flag_filter < clap_verbosity_flag::LevelFilter::Debug
            && self
                .global_arguments
                .debug_flag
        {
            return Some(LevelFilter::DEBUG);
        }

        verbosity_flag_filter
            .as_str()
            .parse()
            .ok()
    }

    fn is_uncolored(&self) -> bool {
        self.global_arguments
            .plain_flag
            || self
                .global_arguments
                .json_flag
            || self
                .global_arguments
                .no_color_flag
            || env::var(format!(
                "{}_NO_COLOR",
                String::from(*APP_NAME).to_uppercase()
            ))
            .map_or(false, |value| !value.is_empty())
    }

    fn is_json(&self) -> bool {
        self.global_arguments
            .json_flag
    }
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    Dummy {},
}

// region: IMPORTS

use crate::{
    app::{logging::InitLog, LoggingSnafu, RunApp},
    cli::cli_template::CliTemplate,
    AppSnafu, Handle, APP_NAME,
};
use clap::Parser;
use owo_colors::{AnsiColors, OwoColorize};
use snafu::{ResultExt, Snafu};
use std::{any::Any, env};
use tracing::Metadata;
use tracing_subscriber::filter::{filter_fn, FilterFn, LevelFilter};

use self::cli_template::GlobalArguments;

// endregion: IMPORTS

// region: MODULES

mod cli_template {
    #[derive(Parser, Debug)]
    #[command(version, author, about, args_conflicts_with_subcommands = true)]
    pub struct CliTemplate {
        #[clap(flatten)]
        pub global_arguments: GlobalArguments,
    }

    #[derive(Debug, Args)]
    #[clap(args_conflicts_with_subcommands = true)]
    pub struct GlobalArguments {
        #[clap(
            long = "json",
            help = "Output in the JSON format for machine readability and scripting purposes.",
            global = true
        )]
        pub json_flag: bool,

        #[clap(
            long = "plain",
            help = "Output as plain text without extra information, for machine readability and scripting purposes.",
            global = true
        )]
        pub plain_flag: bool,

        #[clap(long = "debug", help = "Output debug messages.", global = true)]
        pub debug_flag: bool,

        #[clap(long = "no-color", help = "Disable output coloring.", global = true)]
        pub no_color_flag: bool,

        #[clap(flatten)]
        pub verbose: Verbosity<InfoLevel>,
    }

    // region: IMPORTS

    use clap::{Args, Parser, Subcommand};
    use clap_verbosity_flag::{InfoLevel, Verbosity};

    // endregion: IMPORTS
}

//endregion: MODULES
