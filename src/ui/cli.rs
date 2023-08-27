pub fn run_cli() -> Result<Option<Box<dyn Any>>, crate::Error> {
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

pub trait CliModifier {
    fn verbosity_filter(&self) -> Option<LevelFilter>;
    fn is_uncolored(&self) -> bool;
    fn is_colored(&self) -> bool {
        !self.is_uncolored()
    }
    fn is_json(&self) -> bool;
}

impl CliModifier for GlobalArgs {
    fn verbosity_filter(&self) -> Option<LevelFilter> {
        if self.plain_flag || self.json_flag {
            return Some(LevelFilter::INFO);
        }

        let verbosity_flag_filter = self
            .verbose
            .log_level_filter();

        if verbosity_flag_filter < clap_verbosity_flag::LevelFilter::Debug && self.debug_flag {
            return Some(LevelFilter::DEBUG);
        }

        verbosity_flag_filter
            .as_str()
            .parse()
            .ok()
    }

    fn is_uncolored(&self) -> bool {
        self.plain_flag
            || self.json_flag
            || self.no_color_flag
            || env::var(format!(
                "{}_NO_COLOR",
                String::from(*app::APP_NAME).to_uppercase()
            ))
            .map_or(false, |value| !value.is_empty())
    }

    fn is_json(&self) -> bool {
        self.json_flag
    }
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    CliDummy {},
}

// region: IMPORTS

use crate::app::{self, config, logging};
use clap::Parser;
use owo_colors::OwoColorize;
use snafu::{ResultExt, Snafu};
use std::{any::Any, env};
use tracing_subscriber::filter::LevelFilter;

// endregion: IMPORTS

// region: MODULES

mod cli_template {
    #[derive(Parser, Debug)]
    #[command(version, author, about, args_conflicts_with_subcommands = true)]
    pub struct CliTemplate {
        #[clap(flatten)]
        pub global_arguments: GlobalArgs,

        #[clap(subcommand)]
        pub command: Option<ActionCommand>,

        #[clap(flatten)]
        pub arguments: ListActionArguments,
    }

    #[derive(Clone, Debug, Args)]
    #[clap(args_conflicts_with_subcommands = true, next_display_order = usize::MAX - 100)]
    pub struct GlobalArgs {
        #[clap(
            long = "config",
            short = 'c',
            help = "Path to the configuration file to use.",
            global = true,
            display_order = usize::MAX - 5
        )]
        pub config_file: Option<PathBuf>,

        #[clap(
            long = "json",
            help = "Output in the JSON format for machine readability and scripting purposes.",
            global = true,
            display_order = usize::MAX - 4
        )]
        pub json_flag: bool,

        #[clap(
            long = "plain",
            help = "Output as plain text without extra information, for machine readability and scripting purposes.",
            global = true,
            display_order = usize::MAX - 3
        )]
        pub plain_flag: bool,

        #[clap(
            long = "debug",
            help = "Output debug messages.",
            global = true,
            display_order = usize::MAX - 2
        )]
        pub debug_flag: bool,

        #[clap(
            long = "no-color",
            help = "Disable output coloring.",
            global = true,
            display_order = usize::MAX - 1
        )]
        pub no_color_flag: bool,

        #[clap(flatten)]
        pub verbose: Verbosity<InfoLevel>,
    }

    #[derive(Debug, Subcommand)]
    #[clap(args_conflicts_with_subcommands = true)]
    pub enum ActionCommand {
        #[clap(name = "list", about = "List installed drivers.", display_order = 1)]
        List(ListActionArguments),

        #[clap(
            name = "search",
            about = "Search for available drivers.",
            display_order = 2
        )]
        Search(SearchActionArguments),

        #[clap(name = "install", about = "Install Drivers.", display_order = 3)]
        Install(InstallActionArguments),

        #[clap(
            name = "generate-database",
            aliases = ["generate-db", "gen-db", "gendb"],
            about = "Generate database from input file.", 
            display_order = 4
        )]
        GenerateDatabase(GenerateDatabaseActionArguments),
    }

    #[derive(Debug, Args)]
    pub struct ListActionArguments {
        #[clap(
            value_enum,
            help = "The hardware to list installed drivers for.",
            display_order = 1
        )]
        pub hardware: Option<HardwareKind>,

        #[clap(
            long = "tags",
            alias = "tag",
            short = 't',
            help = "Tag(s) to filter drivers.",
            display_order = 2
        )]
        pub tags: Vec<String>,

        #[clap(
            long = "id",
            short = 'i',
            alias = "driver-id",
            help = "ID to select a driver to look at.",
            display_order = 3
        )]
        pub driver_id: Option<String>,

        #[clap(
            long = "database",
            alias = "db",
            help = "Path to the database file to use for recognizing drivers.",
            display_order = 4
        )]
        pub database_file: Option<PathBuf>,
    }

    #[derive(Debug, Args)]
    pub struct SearchActionArguments {
        #[clap(
            value_enum,
            help = "The hardware to search drivers for.",
            display_order = 1
        )]
        pub hardware: Option<HardwareKind>,

        #[clap(
            long = "tags",
            alias = "tag",
            short = 't',
            help = "Tag(s) to filter drivers.",
            display_order = 2
        )]
        pub tags: Vec<String>,

        #[clap(
            long = "id",
            short = 'i',
            alias = "driver-id",
            help = "ID to select a driver to look for.",
            display_order = 3
        )]
        pub driver_id: Option<String>,

        #[clap(
            long = "database",
            alias = "db",
            help = "Path to the database file to use for searching drivers.",
            display_order = 4
        )]
        pub database_file: Option<PathBuf>,
    }

    #[derive(Debug, Args)]
    pub struct InstallActionArguments {
        #[clap(
            value_enum,
            help = "The hardware to list installed drivers for.",
            display_order = 1
        )]
        pub hardware: Option<HardwareKind>,

        #[clap(
            long = "enable-aur",
            alias = "aur",
            help = "Enable installing from the Arch User Repository (AUR).",
            display_order = 2
        )]
        pub enable_aur: bool,

        #[clap(
            long = "tags",
            alias = "tag",
            short = 't',
            help = "Tag(s) to filter drivers.",
            display_order = 3
        )]
        pub tags: Vec<String>,

        #[clap(
            long = "id",
            short = 'i',
            alias = "driver-id",
            help = "ID to select a driver to look for.",
            display_order = 4
        )]
        pub driver_id: Option<String>,

        #[clap(
            long = "database",
            alias = "db",
            help = "Path to the database file to use for recognizing drivers.",
            display_order = 5
        )]
        pub database_file: Option<PathBuf>,
    }

    #[derive(Debug, Args)]
    pub struct GenerateDatabaseActionArguments {
        #[clap(
            help = "Path to the human-readable input file (Only the YAML format is currently supported).",
            display_order = 1
        )]
        pub input_file: PathBuf,

        #[clap(help = "Path to the database file to generate.", display_order = 2)]
        pub database_file: Option<PathBuf>,
    }

    // region: IMPORTS

    use crate::data::HardwareKind;
    use clap::{Args, Parser, Subcommand};
    use clap_verbosity_flag::{InfoLevel, Verbosity};
    use std::path::PathBuf;

    // endregion: IMPORTS
}

// endregion: MODULES

// region: RE-EXPORTS

pub use cli_template::*;

// endregion: RE-EXPORTS
