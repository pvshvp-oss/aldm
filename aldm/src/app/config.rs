/// A trait to be implemented by configuration structs. Any assignable fields
/// must all have their types wrapped in `Option`, for example, `Option<bool>`.
pub trait Configuration: Default + DeserializeOwned {
    /// A method that returns a new configuration `struct` with all assignable
    /// fields set to `None`. Any assignable fields not set to None will not
    /// be loaded/replaced.
    fn new() -> Self;

    /// Replace any unassigned fields (which have the value `None`) from the
    /// supplied environmental variables if the relevant environmental
    /// variables are set. This method must call `self.set_loaded()` if any
    /// fields were set/modified.
    fn env(&mut self) -> &mut Self;

    /// Replace any unassigned fields (which have the value `None`) from a
    /// config filepath if the file at the supplied filepath is valid and has
    /// the relevant fields set. Invalid paths fail quietly, allowing one to
    /// supply preferred config filepaths. However, invalid file formats will
    /// return an error. This method must call `self.set_loaded()` if any
    /// fields were set/modified.
    fn filepath(&mut self, config_filepath: impl Into<PathBuf>) -> Result<&mut Self, Error> {
        let config_filepath: PathBuf = config_filepath.into();

        if permissions::is_writable(&config_filepath).unwrap_or(false) {
            let config_file = File::open(config_filepath.clone()).context(ReadConfigFileSnafu {
                path: config_filepath.clone(),
            })?;
            let config_file_reader = BufReader::new(config_file);

            let other_config =
                serde_yaml::from_reader(config_file_reader).context(ConfigFileFormatSnafu {
                    path: config_filepath.clone(),
                })?;

            self.config(other_config);

            self.set_loaded();
        }

        Ok(self)
    }

    /// Like [`filepath()`], but the supplied filepath is a variant of `Option`
    fn optional_filepath(
        &mut self,
        optional_config_filepath: Option<impl Into<PathBuf>>,
    ) -> Result<&mut Self, Error> {
        if let Some(config_filepath) = optional_config_filepath {
            self.filepath(config_filepath)?;
        }

        Ok(self)
    }

    /// Replace any unassigned fields (which have the value `None`) from the
    /// supplied config struct if that struct has the relevant fields set.
    /// This method must call `self.set_loaded()` if any fields were
    /// set/modified.
    fn config(&mut self, other: Self) -> &mut Self;

    /// Like [`config()`], but the supplied config struct is a variant of
    /// `Option`
    fn optional_config(&mut self, optional_other: Option<Self>) -> &mut Self {
        if let Some(other) = optional_other {
            self.config(other);
        }

        self
    }

    /// Call to ensure that the configuration struct is assigned default values
    /// if loading fields was not successful from all sources tried (for
    /// example, through environment variables, through a config filepath,
    /// through a different config struct, etc.)
    fn ensure_loaded(&mut self) -> &mut Self {
        if !self.is_loaded() {
            *self = Self::default();
        }

        self
    }

    /// Method to call to notify/record that the configuration has been loaded
    /// from any source (for example, through environment variables, through a
    /// config filepath, through a different config struct, etc.)
    fn set_loaded(&mut self);

    /// Method to call to determine if the configuration has already been
    /// loaded at least once from any source (for example, through environment
    /// variables, through a config filepath, through a different
    /// config struct, etc.)
    fn is_loaded(&self) -> bool;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub log_dirpath: Option<PathBuf>,

    pub log_level_filter: Option<log::LevelFilter>,

    pub no_color: Option<bool>,

    #[serde(skip)]
    is_modified: bool,
}

impl Configuration for Config {
    fn new() -> Self {
        Self {
            log_dirpath: None,
            log_level_filter: None,
            no_color: None,
            is_modified: false,
        }
    }

    fn env(&mut self) -> &mut Self {
        let env_config = Self {
            log_dirpath: env::var("LOG_DIRPATH")
                .map(|s| s.into())
                .ok(),
            log_level_filter: env::var("LOG_LEVEL_FILTER")
                .and_then(|s| {
                    s.parse()
                        .map_err(|_| env::VarError::NotPresent)
                })
                .ok(),
            no_color: env::var("NO_COLOR")
                .and_then(|s| {
                    s.parse()
                        .map_err(|_| env::VarError::NotPresent)
                })
                .ok(),
            is_modified: false,
        };

        self.config(env_config);

        self.set_loaded();

        self
    }

    fn config(&mut self, other: Self) -> &mut Self {
        self.log_dirpath = self
            .log_dirpath
            .take()
            .or(other.log_dirpath);
        self.log_level_filter = self
            .log_level_filter
            .take()
            .or(other.log_level_filter);
        self.no_color = self
            .no_color
            .take()
            .or(other.no_color);

        self.set_loaded();

        self
    }

    fn set_loaded(&mut self) {
        self.is_modified = true;
    }

    fn is_loaded(&self) -> bool {
        self.is_modified
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            log_dirpath: None,
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
        display("could not read the config file at {:?}: {source}", path),
        visibility(pub)
    )]
    ReadConfigFile {
        path: PathBuf,
        source: std::io::Error,
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

use std::{env, fs::File, io::BufReader, path::PathBuf};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

// endregion: IMPORTS
