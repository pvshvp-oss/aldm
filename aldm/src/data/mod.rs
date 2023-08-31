#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Readable, Writable,
)]
pub enum HardwareKind {
    #[serde(deserialize_with = "deserialize_struct_case_insensitive")]
    #[serde(alias = "graphics", alias = "gpu")]
    Graphics,

    #[serde(deserialize_with = "deserialize_struct_case_insensitive")]
    #[serde(alias = "ethernet", alias = "lan")]
    Ethernet,

    #[serde(deserialize_with = "deserialize_struct_case_insensitive")]
    #[serde(alias = "wireless", alias = "wifi")]
    Wireless,

    #[serde(deserialize_with = "deserialize_struct_case_insensitive")]
    #[serde(alias = "sound", alias = "audio")]
    Audio,
}

impl FromStr for HardwareKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let user_string = String::from(s).to_lowercase();
        match user_string
            .to_ascii_lowercase()
            .as_str()
        {
            "graphics" | "gpu" => Ok(HardwareKind::Graphics),
            "ethernet" | "lan" => Ok(HardwareKind::Ethernet),
            "wireless" | "wifi" => Ok(HardwareKind::Wireless),
            "sound" | "audio" => Ok(HardwareKind::Audio),
            _ => Err(Error::InvalidVariant {
                value: s.into(),
                enum_name: "HardwareKind".into(),
                allowed_values: vec![
                    "graphics", "gpu", "ethernet", "lan", "wireless", "wifi", "sound", "audio",
                ]
                .into_iter()
                .map(|s| String::from(s))
                .collect(),
            }),
        }
    }
}

impl fmt::Display for HardwareKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HardwareKind::Graphics => write!(f, "Graphics"),
            HardwareKind::Ethernet => write!(f, "Ethernet"),
            HardwareKind::Wireless => write!(f, "Wireless"),
            HardwareKind::Audio => write!(f, "Audio"),
        }
    }
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(
        display("'{value}' is not recognized as a variant of '{enum_name}'. The allowed values are {allowed_values:?}")
    )]
    InvalidVariant {
        value: String,
        enum_name: String,
        allowed_values: Vec<String>,
    },
}

// region: IMPORTS

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use snafu::Snafu;
use speedy::{Readable, Writable};
use std::{fmt, str::FromStr};

// endregion: IMPORTS

// region: MODULES

pub mod database;
pub mod input_file;

// endregion: MODULES

// region: RE-EXPORTS

pub use database::*;
pub use input_file::*;

// endregion: RE-EXPORTS
