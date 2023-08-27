pub fn init_config() -> Result<Config, Error> {
    let config_filename = format!("{}.conf", *app::APP_NAME);
    let xdg_app_dirs = xdg::BaseDirectories::with_prefix(*app::APP_NAME)
        .context(RetreiveConfigUserAppBaseDirectoriesSnafu {})?;
    let candidate_config_filepaths = vec![
        xdg_app_dirs.get_config_file(&config_filename),
        format!("/etc/{}", config_filename).into(),
        format!("/var/tmp/{}/{}", *app::APP_NAME, config_filename).into(),
    ];
    let config_filepath = app::first_valid_path(&candidate_config_filepaths);
    let config_filepath = match config_filepath {
        Some(p) => p
            .as_ref()
            .to_owned(),
        None => xdg_app_dirs
            .place_config_file(config_filename)
            .context(CreateConfigDirectorySnafu {
                path: {
                    let mut config_dir_path = xdg_app_dirs.get_config_home();
                    config_dir_path.push(*app::APP_NAME);
                    config_dir_path
                },
            })?,
    };
    if config_filepath.exists() {
        let config_file = fs::File::open(config_filepath.clone()).context(ReadConfigFileSnafu {
            path: config_filepath.clone(),
        })?;
        Ok(
            serde_yaml::from_reader(BufReader::new(config_file)).context(
                ConfigFileFormatSnafu {
                    path: config_filepath.clone(),
                },
            )?,
        )
    } else {
        let config_file =
            fs::File::create(config_filepath.clone()).context(CreateConfigFileSnafu {
                path: config_filepath.clone(),
            })?;
        let default_config = Config::default();
        let buf_writer = BufWriter::new(config_file);
        serde_yaml::to_writer(buf_writer, &default_config).context(WriteConfigFileSnafu {
            path: config_filepath.clone(),
        })?;
        Ok(default_config)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Readable, Writable)]
pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Config {}
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
    fs,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use crate::app;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use speedy::{Readable, Writable};

// endregion: IMPORTS
