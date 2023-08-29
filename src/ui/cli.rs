pub fn run_cli() -> Result<Box<dyn any::Any>, crate::Error> {
    let (_cli_input, _worker_guards) = ui::run_common::<CliTemplate>()?;

    tracing::debug!(
        "Running in {} mode... {}",
        "CLI".blue(),
        console::Emoji("🔤", "")
    );

    Ok(Box::new(()))
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    CliDummy {},
}

// region: IMPORTS
use crate::ui;
use owo_colors::OwoColorize;
use snafu::Snafu;
use std::any;

// endregion: IMPORTS

// region: MODULES

mod cli_template {
    #[derive(Parser, Debug)]
    #[command(version, author, about, args_conflicts_with_subcommands = true)]
    pub struct CliTemplate {
        #[clap(flatten)]
        pub global_args: GlobalArgs<clap_verbosity_flag::InfoLevel>,

        #[clap(subcommand)]
        pub command: Option<ActionCommand>,

        #[clap(flatten)]
        pub arguments: ListActionArguments,
    }

    impl ui::GlobalArguments for CliTemplate {
        type L = clap_verbosity_flag::InfoLevel;

        fn config_file(&self) -> &Option<PathBuf> {
            &self
                .global_args
                .config_file
        }

        fn is_json(&self) -> bool {
            self.global_args
                .json_flag
        }

        fn is_plain(&self) -> bool {
            self.global_args
                .plain_flag
        }

        fn is_debug(&self) -> bool {
            self.global_args
                .debug_flag
        }

        fn is_no_color(&self) -> bool {
            self.global_args
                .no_color_flag
        }

        fn is_test(&self) -> bool {
            self.global_args
                .test_flag
        }

        fn verbosity(&self) -> &clap_verbosity_flag::Verbosity<Self::L> {
            &self
                .global_args
                .verbose
        }
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

    use crate::{
        data::HardwareKind,
        ui::{self, GlobalArgs},
    };
    use clap::{Args, Parser, Subcommand};
    use std::path::PathBuf;

    // endregion: IMPORTS
}

// endregion: MODULES

// region: RE-EXPORTS

pub use cli_template::*;

// endregion: RE-EXPORTS
