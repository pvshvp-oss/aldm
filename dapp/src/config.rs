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

    #[cfg(feature = "serde")]
    /// Replace any unassigned fields (which have the value `None`) from a
    /// config filepath if the file at the supplied filepath is valid and has
    /// the relevant fields set. Files which do not exist, or which cannot be
    /// read, or which have invalid formats should return an error. This method
    /// must call `self.set_loaded()` if any fields were set/modified.
    fn try_filepath<'de, P, D>(&mut self, config_filepath: P) -> Result<&mut Self, Error>
    where
        P: AsRef<Path>,
        Self: Deserialize<'de>,
        D: Deserializer<'de> + From<BufReader<File>>,
    {
        let config_filepath = config_filepath.as_ref();
        if !config_filepath.exists() {
            Err(Error::FindConfigFile {
                path: config_filepath.clone(),
            })
        } else if !config_filepath.is_readable() {
            Err(Error::ReadConfigFile {
                path: config_filepath.clone(),
                source: std::io::ErrorKind::PermissionDenied.into(),
            })
        } else {
            let file = File::open(config_filepath.as_ref()).context(ReadConfigFileSnafu {
                path: config_filepath.clone(),
            })?;
            let file_reader = BufReader::new(file);
            let other_config =
                Self::deserialize(D::from(file_reader)).context(ParseConfigFileSnafu {
                    path: config_filepath.clone(),
                })?;
            self.config(other_config);
            self.set_loaded();
            Ok(self)
        }
    }

    #[cfg(feature = "serde")]
    /// Like [`try_filepath()`], but takes an optional filepath.
    fn try_optional_filepath<'de, P, D>(
        &mut self,
        optional_config_filepath: Option<P>,
    ) -> Result<&mut Self, Error>
    where
        P: AsRef<Path>,
        Self: Deserialize<'de>,
        D: Deserializer<'de> + From<BufReader<File>>,
    {
        match optional_config_filepath {
            Some(config_filepath) => self.try_filepath::<'de, P, D>(config_filepath),
            None => Err(Error::FindOptionalConfigFile {
                path: optional_config_filepath.clone(),
            }),
        }
    }

    #[cfg(feature = "serde")]
    /// Like [`try_filepath()`], but invalid paths fail quietly, allowing one
    /// to use preferred config filepaths that may or may not exist. Invalid
    /// file formats still result in an error.
    fn filepath<'de, P, D>(&mut self, config_filepath: P) -> Result<&mut Self, Error>
    where
        P: AsRef<Path>,
        Self: Deserialize<'de>,
        D: Deserializer<'de> + From<BufReader<File>>,
    {
        let result = self.try_filepath::<'de, P, D>(config_filepath);
        match result {
            Ok(_) => result,
            Err(err) => match err {
                Error::FindConfigFile { .. } | Error::ReadConfigFile { .. } => Ok(self),
                _ => Err(err),
            },
        }
    }

    #[cfg(feature = "serde")]
    /// Like [`filepath()`], but takes an optional filepath
    fn optional_filepath<'de, P, D>(
        &mut self,
        optional_config_filepath: Option<P>,
    ) -> Result<&mut Self, Error>
    where
        P: AsRef<Path>,
        Self: Deserialize<'de>,
        D: Deserializer<'de> + From<BufReader<File>>,
    {
        match optional_config_filepath {
            Some(config_filepath) => self.filepath::<'de, P, D>(config_filepath),
            None => Ok(self),
        }
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

#[cfg(feature = "serde")]
#[delegatable_trait_remote]
trait Deserializer<'de>: Sized {
    /// The error type that can be returned if some error occurs during
    /// deserialization.
    type Error: Error;

    /// Require the `Deserializer` to figure out how to drive the visitor based
    /// on what data type is in the input.
    ///
    /// When implementing `Deserialize`, you should avoid relying on
    /// `Deserializer::deserialize_any` unless you need to be told by the
    /// Deserializer what type is in the input. Know that relying on
    /// `Deserializer::deserialize_any` means your data type will be able to
    /// deserialize from self-describing formats only, ruling out Postcard and
    /// many others.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a `bool` value.
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting an `i8` value.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting an `i16` value.
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting an `i32` value.
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting an `i64` value.
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting an `i128` value.
    ///
    /// The default behavior unconditionally returns an error.
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        Err(Error::custom("i128 is not supported"))
    }

    /// Hint that the `Deserialize` type is expecting a `u8` value.
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a `u16` value.
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a `u32` value.
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a `u64` value.
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting an `u128` value.
    ///
    /// The default behavior unconditionally returns an error.
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        Err(Error::custom("u128 is not supported"))
    }

    /// Hint that the `Deserialize` type is expecting a `f32` value.
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a `f64` value.
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a `char` value.
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a string value and does
    /// not benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    ///
    /// If the `Visitor` would benefit from taking ownership of `String` data,
    /// indicate this to the `Deserializer` by using `deserialize_string`
    /// instead.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a string value and would
    /// benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    ///
    /// If the `Visitor` would not benefit from taking ownership of `String`
    /// data, indicate that to the `Deserializer` by using `deserialize_str`
    /// instead.
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a byte array and does not
    /// benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    ///
    /// If the `Visitor` would benefit from taking ownership of `Vec<u8>` data,
    /// indicate this to the `Deserializer` by using `deserialize_byte_buf`
    /// instead.
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a byte array and would
    /// benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    ///
    /// If the `Visitor` would not benefit from taking ownership of `Vec<u8>`
    /// data, indicate that to the `Deserializer` by using `deserialize_bytes`
    /// instead.
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting an optional value.
    ///
    /// This allows deserializers that encode an optional value as a nullable
    /// value to convert the null value into `None` and a regular value into
    /// `Some(value)`.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a unit value.
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a unit struct with a
    /// particular name.
    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a newtype struct with a
    /// particular name.
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a sequence of values.
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a sequence of values and
    /// knows how many values there are without looking at the serialized data.
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a tuple struct with a
    /// particular name and number of fields.
    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a map of key-value pairs.
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a struct with a particular
    /// name and fields.
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting an enum value with a
    /// particular name and possible variants.
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting the name of a struct
    /// field or the discriminant of an enum variant.
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Hint that the `Deserialize` type needs to deserialize a value whose type
    /// doesn't matter because it is ignored.
    ///
    /// Deserializers for non-self-describing formats may not support this mode.
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Determine whether `Deserialize` implementations should expect to
    /// deserialize their human-readable form.
    ///
    /// Some types have a human-readable form that may be somewhat expensive to
    /// construct, as well as a binary form that is compact and efficient.
    /// Generally text-based formats like JSON and YAML will prefer to use the
    /// human-readable one and binary formats like Postcard will prefer the
    /// compact one.
    ///
    /// ```edition2021
    /// # use std::ops::Add;
    /// # use std::str::FromStr;
    /// #
    /// # struct Timestamp;
    /// #
    /// # impl Timestamp {
    /// #     const EPOCH: Timestamp = Timestamp;
    /// # }
    /// #
    /// # impl FromStr for Timestamp {
    /// #     type Err = String;
    /// #     fn from_str(_: &str) -> Result<Self, Self::Err> {
    /// #         unimplemented!()
    /// #     }
    /// # }
    /// #
    /// # struct Duration;
    /// #
    /// # impl Duration {
    /// #     fn seconds(_: u64) -> Self { unimplemented!() }
    /// # }
    /// #
    /// # impl Add<Duration> for Timestamp {
    /// #     type Output = Timestamp;
    /// #     fn add(self, _: Duration) -> Self::Output {
    /// #         unimplemented!()
    /// #     }
    /// # }
    /// #
    /// use serde::de::{self, Deserialize, Deserializer};
    ///
    /// impl<'de> Deserialize<'de> for Timestamp {
    ///     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    ///     where
    ///         D: Deserializer<'de>,
    ///     {
    ///         if deserializer.is_human_readable() {
    ///             // Deserialize from a human-readable string like "2015-05-15T17:01:00Z".
    ///             let s = String::deserialize(deserializer)?;
    ///             Timestamp::from_str(&s).map_err(de::Error::custom)
    ///         } else {
    ///             // Deserialize from a compact binary representation, seconds since
    ///             // the Unix epoch.
    ///             let n = u64::deserialize(deserializer)?;
    ///             Ok(Timestamp::EPOCH + Duration::seconds(n))
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// The default implementation of this method returns `true`. Data formats
    /// may override this to `false` to request a compact form for types that
    /// support one. Note that modifying this method to change a format from
    /// human-readable to compact or vice versa should be regarded as a breaking
    /// change, as a value serialized in human-readable mode is not required to
    /// deserialize from the same data in compact mode.
    #[inline]
    fn is_human_readable(&self) -> bool {
        true
    }

    // Not public API.
    #[cfg(all(not(no_serde_derive), any(feature = "std", feature = "alloc")))]
    #[doc(hidden)]
    fn __deserialize_content<V>(
        self,
        _: crate::actually_private::T,
        visitor: V,
    ) -> Result<crate::__private::de::Content<'de>, Self::Error>
    where
        V: Visitor<'de, Value = crate::__private::de::Content<'de>>,
    {
        self.deserialize_any(visitor)
    }
}

#[derive(Delegate)]
#[delegate(Deserializer<'_>)]
pub enum Format<'a> {
    #[cfg(feature = "yaml")]
    Yaml(serde_yaml::Deserializer<'a>),
}

trait SizedSerdeError: serde::de::Error + Sized {}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("could not find a config file at {:?}", path), visibility(pub))]
    FindConfigFile { path: PathBuf },

    #[non_exhaustive]
    #[snafu(
        display("could not find an optional config file at {:?}", optional_path),
        visibility(pub)
    )]
    FindOptionalConfigFile { optional_path: Option<PathBuf> },

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
        display(
            "could not read the optional config file at {:?}: {source}",
            optional_path
        ),
        visibility(pub)
    )]
    ReadOptionalConfigFile {
        optional_path: Option<PathBuf>,
        source: std::io::Error,
    },

    #[cfg(feature = "serde")]
    #[non_exhaustive]
    #[snafu(
        display("The config file at {:?} has incorrect format: {source}", path),
        visibility(pub)
    )]
    ParseConfigFile {
        path: PathBuf,
        source: Box<dyn std::error::Error>,
    },
}

// region: IMPORTS

use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

#[cfg(feature = "serde")]
use ambassador::delegatable_trait_remote;

#[cfg(feature = "serde")]
use ambassador::Delegate;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer};

use snafu::{self, ResultExt, Snafu};

use crate::path::ValidPath;

// endregion: IMPORTS
