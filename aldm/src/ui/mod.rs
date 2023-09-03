impl<T> CliModifier for T
where
    T: GlobalArguments,
    <T as GlobalArguments>::L: LogLevel,
{
}

pub trait CliModifier: GlobalArguments
where
    <Self as GlobalArguments>::L: LogLevel,
{
    fn verbosity_filter(&self) -> Option<LevelFilter> {
        let verbosity_flag_filter = self
            .verbosity()
            .log_level_filter();

        if verbosity_flag_filter < clap_verbosity_flag::LevelFilter::Debug && self.is_debug() {
            return Some(LevelFilter::DEBUG);
        }

        verbosity_flag_filter
            .as_str()
            .parse()
            .ok()
    }

    fn is_uncolored(&self) -> bool {
        self.is_plain()
            || self.is_json()
            || self.is_no_color()
            || env::var(format!(
                "{}_NO_COLOR",
                String::from(*app::APP_NAME).to_uppercase()
            ))
            .map_or(false, |value| !value.is_empty())
    }

    fn is_colored(&self) -> bool {
        !self.is_uncolored()
    }
}

pub trait GlobalArguments {
    type L;

    fn config_file(&self) -> &Option<PathBuf>;

    fn is_json(&self) -> bool;

    fn is_plain(&self) -> bool;

    fn is_debug(&self) -> bool;

    fn is_no_color(&self) -> bool;

    fn is_test(&self) -> bool;

    fn verbosity(&self) -> &clap_verbosity_flag::Verbosity<Self::L>
    where
        Self::L: LogLevel;
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""))]
    Dummy {},
}

// region: IMPORTS

use clap_verbosity_flag::LogLevel;
use core::fmt;
use owo_colors::OwoColorize;
use snafu::{ResultExt, Snafu};
use std::{env, path::PathBuf};
use tracing_subscriber::filter::LevelFilter;

use crate::app::{
    self,
    config::{self, Configuration},
    logging,
};

// endregion: IMPORTS

// region: MODULES

mod cli_template {
    #[derive(Clone, Debug, Args)]
    #[clap(args_conflicts_with_subcommands = true, next_display_order = usize::MAX - 100)]
    pub struct GlobalArgs<L>
    where
        L: clap_verbosity_flag::LogLevel,
    {
        #[clap(
            long = "config",
            short = 'c',
            help = "Path to the configuration file to use.",
            global = true,
            display_order = usize::MAX - 6
        )]
        pub config_file: Option<PathBuf>,

        #[clap(
            long = "json",
            help = "Output in the JSON format for machine readability and scripting purposes.",
            global = true,
            display_order = usize::MAX - 5
        )]
        pub json_flag: bool,

        #[clap(
            long = "plain",
            help = "Output as plain text without extra information, for machine readability and scripting purposes.",
            global = true,
            display_order = usize::MAX - 4
        )]
        pub plain_flag: bool,

        #[clap(
            long = "debug",
            help = "Output debug messages.",
            global = true,
            display_order = usize::MAX - 3
        )]
        pub debug_flag: bool,

        #[clap(
            long = "no-color",
            help = "Disable output coloring.",
            global = true,
            display_order = usize::MAX - 2
        )]
        pub no_color_flag: bool,

        #[clap(
            long = "test",
            help = "Avoid destructive modifications and show all output subject to the commandline filters. Useful for dry-runs and for developers.",
            global = true,
            display_order = usize::MAX - 1
        )]
        pub test_flag: bool,

        #[clap(flatten)]
        pub verbose: Verbosity<L>,
    }

    // region: IMPORTS

    use clap::Args;
    use clap_verbosity_flag::Verbosity;
    use std::path::PathBuf;

    // endregion: IMPORTS
}

// endregion: MODULES

// region: RE-EXPORTS

pub use cli_template::*;

// endregion: RE-EXPORTS
