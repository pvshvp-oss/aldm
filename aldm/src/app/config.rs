pub fn init_config() -> Result<(Config, Vec<PathBuf>), Error> {
    let config_filename = format!("{}.conf", *app::APP_NAME);
    let xdg_app_dirs = xdg::BaseDirectories::with_prefix(*app::APP_NAME)
        .context(RetreiveConfigUserAppBaseDirectoriesSnafu {})?;
    let default_config_filepath = xdg_app_dirs.get_config_file(&config_filename);
    let candidate_config_filepaths = vec![
        default_config_filepath.clone(),
        format!("/etc/{}/{}", *app::APP_NAME, config_filename).into(),
        format!("/etc/{}", config_filename).into(),
        format!("/var/tmp/{}/{}", *app::APP_NAME, config_filename).into(),
    ];

    let mut config = Config::new();

    let readable_config_filepaths = candidate_config_filepaths
        .iter()
        .all_readable_paths();
    for readable_config_filepath in readable_config_filepaths.clone() {
        println!("{:?}", readable_config_filepath);
        let config_file =
            fs::File::open(readable_config_filepath.clone()).context(ReadConfigFileSnafu {
                path: readable_config_filepath.clone(),
            })?;
        config.obtain_unassigned_from(
            serde_yaml::from_reader(BufReader::new(config_file)).context(
                ConfigFileFormatSnafu {
                    path: readable_config_filepath.clone(),
                },
            )?,
        );
        println!("{:#?}", config);
    }

    let readable_config_filepaths = readable_config_filepaths.map(|p| p.clone());
    if config.is_modified() {
        Ok((config, readable_config_filepaths.collect::<Vec<PathBuf>>()))
    } else {
        Ok((
            Config::default(),
            readable_config_filepaths.collect::<Vec<PathBuf>>(),
        ))
    }
}

pub fn create_config_file(config: &Config, config_filepath: &PathBuf) -> Result<(), Error> {
    serde_yaml::to_writer(
        BufWriter::new(
            File::create(config_filepath.clone()).context(CreateConfigFileSnafu {
                path: config_filepath.clone(),
            })?,
        ),
        &config,
    )
    .context(WriteConfigFileSnafu {
        path: config_filepath.clone(),
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub log_directory: Option<String>,

    pub log_level_filter: Option<log::LevelFilter>,

    pub no_color: Option<bool>,

    #[serde(skip)]
    is_modified: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            log_directory: None,
            log_level_filter: None,
            no_color: None,
            is_modified: false,
        }
    }

    pub fn obtain_unassigned_from(&mut self, other: Self) {
        self.log_directory = self
            .log_directory
            .take()
            .or(other.log_directory);
        self.log_level_filter = self
            .log_level_filter
            .take()
            .or(other.log_level_filter);
        self.no_color = self
            .no_color
            .take()
            .or(other.no_color);
        self.is_modified = true;
    }

    pub fn is_modified(&self) -> bool {
        self.is_modified
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            log_directory: None,
            log_level_filter: Some(log::LevelFilter::Info),
            no_color: Some(false),
            is_modified: true,
        }
    }
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(
        display("could not retreive the XDG base directories for the user: {source}"),
        visibility(pub)
    )]
    RetreiveConfigUserAppBaseDirectories { source: xdg::BaseDirectoriesError },

    #[non_exhaustive]
    #[snafu(
        display("could not create the config directory at {:?}: {source}", path),
        visibility(pub)
    )]
    CreateConfigDirectory {
        path: PathBuf,
        source: std::io::Error,
    },

    #[non_exhaustive]
    #[snafu(
        display("could not create the config file at {:?}: {source}", path),
        visibility(pub)
    )]
    CreateConfigFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[non_exhaustive]
    #[snafu(
        display("could not read the config file at {:?}: {source}", path),
        visibility(pub)
    )]
    ReadConfigFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[non_exhaustive]
    #[snafu(
        display("could not write a config file at {:?}: {source}", path),
        visibility(pub)
    )]
    WriteConfigFile {
        path: PathBuf,
        source: serde_yaml::Error,
    },

    #[non_exhaustive]
    #[snafu(
        display("The config file at {:?} has incorrect format: {source}", path),
        visibility(pub)
    )]
    ConfigFileFormat {
        path: PathBuf,
        source: serde_yaml::Error,
    },
}

// region: IMPORTS

use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use crate::app::{self, PathListPermissions};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

// endregion: IMPORTS
