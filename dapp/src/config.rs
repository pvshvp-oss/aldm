/// A trait to be implemented by configuration structs. Any assignable fields
/// must be of an optional type, like for example, `Option<bool>`, or
/// `Option<PathBuf>`.
pub trait Configuration: Default {
    /// A method that should return a new configuration `struct` with all
    /// assignable fields set to `None`. Any assignable fields not set to None
    /// should not be loaded/replaced by other implemented methods of this
    /// trait.
    fn new() -> Self;

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

    /// Replace any unassigned fields (which have the value `None`) from the
    /// supplied environmental variables if the relevant environmental
    /// variables are set. This method must call `self.set_loaded()` if any
    /// fields were set/modified.
    fn env(&mut self) -> &mut Self;

    /// Replace any unassigned fields (which have the value `None`) from a
    /// config filepath if the file at the supplied filepath is valid and has
    /// the relevant fields set. Files which do not exist, or which cannot be
    /// read, or which have invalid formats should return an error. This method
    /// must call `self.set_loaded()` if any fields were set/modified.
    fn try_filepath<P>(
        &mut self,
        config_filepath: P,
        parse_fn: fn(filepath: P) -> Result<Self, Error>,
    ) -> Result<&mut Self, Error>
    where
        P: AsRef<Path>,
    {
        snafu::ensure!(
            config_filepath.is_readable(),
            FindConfigFileSnafu {
                path: config_filepath.as_ref(),
            }
        );
        let other_config = parse_fn(config_filepath)?;
        self.config(other_config);
        self.set_loaded();
        Ok(self)
    }

    /// Like [`try_filepath()`], but takes an optional filepath.
    fn try_optional_filepath<P>(
        &mut self,
        optional_config_filepath: Option<P>,
        parse_fn: fn(filepath: P) -> Result<Self, Error>,
    ) -> Result<&mut Self, Error>
    where
        P: AsRef<Path>,
    {
        let invalid_filepath_error = Error::FindOptionalConfigFile {
            path: optional_config_filepath
                .as_ref()
                .map(|p| {
                    p.as_ref()
                        .to_owned()
                }),
        };
        if optional_config_filepath.is_readable() {
            let unwrapped_filepath = optional_config_filepath.ok_or(invalid_filepath_error)?;
            let other_config = parse_fn(unwrapped_filepath)?;
            self.config(other_config);
            self.set_loaded();
            Ok(self)
        } else {
            Err(invalid_filepath_error)
        }
    }

    /// Like [`try_filepath()`], but invalid paths fail quietly, allowing one
    /// to use preferred config filepaths that may or may not exist. Invalid
    /// file formats still result in an error.
    fn filepath<P>(
        &mut self,
        config_filepath: P,
        parse_fn: fn(filepath: P) -> Result<Self, Error>,
    ) -> Result<&mut Self, Error>
    where
        P: AsRef<Path>,
    {
        let result = self.try_filepath(config_filepath, parse_fn);
        let modified_result = match result {
            Ok(_) => Ok(self),
            Err(err) => match err {
                Error::FindConfigFile { .. } => Ok(self),
                _ => Err(err),
            },
        };
        modified_result
    }

    /// Like [`filepath()`], but takes an optional filepath
    fn optional_filepath<P>(
        &mut self,
        optional_config_filepath: Option<P>,
        parse_fn: fn(filepath: P) -> Result<Self, Error>,
    ) -> Result<&mut Self, Error>
    where
        P: AsRef<Path>,
    {
        let result = self.try_optional_filepath(optional_config_filepath, parse_fn);
        let modified_result = match result {
            Ok(_) => Ok(self),
            Err(err) => match err {
                Error::FindOptionalConfigFile { .. } => Ok(self),
                _ => Err(err),
            },
        };
        modified_result
    }

    #[cfg(feature = "serde_yaml")]
    fn yaml_parse_fn<P>(filepath: P) -> Result<Self, Error>
    where
        Self: serde::de::DeserializeOwned,
        P: AsRef<Path>,
    {
        use std::{fs::File, io::BufReader};

        use snafu::ResultExt;

        let file = File::open(filepath.as_ref()).context(ReadConfigFileSnafu {
            path: filepath.as_ref(),
        })?;
        let file_reader = BufReader::new(file);
        let other_config = serde_yaml::from_reader(file_reader).context(ParseConfigFileSnafu {
            path: filepath.as_ref(),
        })?;
        Ok(other_config)
    }

    #[cfg(feature = "serde_yaml")]
    /// Like [`try_filepath()`], but the supplied filepath has the YAML format
    fn try_yaml_filepath<P>(&mut self, config_filepath: P) -> Result<&mut Self, Error>
    where
        Self: serde::de::DeserializeOwned,
        P: AsRef<Path>,
    {
        self.try_filepath(config_filepath, Self::yaml_parse_fn)
    }

    #[cfg(feature = "serde_yaml")]
    /// Like [`try_optional_filepath()`], but the supplied filepath has the YAML format
    fn try_optional_yaml_filepath<P>(
        &mut self,
        optional_config_filepath: Option<P>,
    ) -> Result<&mut Self, Error>
    where
        Self: serde::de::DeserializeOwned,
        P: AsRef<Path>,
    {
        self.try_optional_filepath(optional_config_filepath, Self::yaml_parse_fn)
    }

    #[cfg(feature = "serde_yaml")]
    /// Like [`filepath()`], but the supplied filepath has the YAML format
    fn yaml_filepath<P>(&mut self, config_filepath: P) -> Result<&mut Self, Error>
    where
        Self: serde::de::DeserializeOwned,
        P: AsRef<Path>,
    {
        self.filepath(config_filepath, Self::yaml_parse_fn)
    }

    #[cfg(feature = "serde_yaml")]
    /// Like [`optional_filepath()`], but the supplied filepath has the YAML format
    fn optional_yaml_filepath<P>(
        &mut self,
        optional_config_filepath: Option<P>,
    ) -> Result<&mut Self, Error>
    where
        Self: serde::de::DeserializeOwned,
        P: AsRef<Path>,
    {
        self.optional_filepath(optional_config_filepath, Self::yaml_parse_fn)
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
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(
        display("could not find a readable config file at {:?}", path),
        visibility(pub)
    )]
    FindConfigFile { path: PathBuf },

    #[non_exhaustive]
    #[snafu(
        display("could not find a optional config file at {:?}", path),
        visibility(pub)
    )]
    FindOptionalConfigFile { path: Option<PathBuf> },

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
    ParseConfigFile {
        path: PathBuf,
        source: serde_yaml::Error,
    },
}

// region: IMPORTS

use std::path::{Path, PathBuf};

use snafu::{self, Snafu};

use crate::permissions::Permissions;

// endregion: IMPORTS
