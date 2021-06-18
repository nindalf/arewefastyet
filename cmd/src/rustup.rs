use std::process::Command;

use anyhow::{anyhow, Context, Result};
use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Copy,
    Clone,
    Hash,
    IntoEnumIterator,
    Serialize,
    PartialOrd,
    Ord,
    Deserialize,
    Eq,
    PartialEq,
)]
pub(crate) enum Version {
    V1_34 = 34,
    V1_35 = 35,
    V1_36 = 36,
    V1_37 = 37,
    V1_38 = 38,
    V1_39 = 39,
    V1_40 = 40,
    V1_41 = 41,
    V1_42 = 42,
    V1_43 = 43,
    V1_44 = 44,
    V1_45 = 45,
    V1_46 = 46,
    V1_47 = 47,
    V1_48 = 48,
    V1_49 = 49,
    V1_50 = 50,
    V1_51 = 51,
    V1_52 = 52,
    V1_53 = 53,
}

pub(crate) fn set_profile_minimal() -> Result<()> {
    rustup(&["set", "profile", "minimal"])?;
    log::info!("Set profile to minimal");
    Ok(())
}

pub(crate) fn set_version(version: Version) -> Result<()> {
    let version = version.get_string();
    rustup(&["toolchain", "install", version])?;
    rustup(&["default", version])?;
    log::info!("Switched to version {}", version);
    Ok(())
}

fn rustup(args: &[&str]) -> Result<()> {
    let output = Command::new("rustup")
        .args(args)
        .output()
        .with_context(|| "failed to execute rustup")?;
    if !output.status.success() {
        let stderr =
            std::str::from_utf8(&output.stderr).with_context(|| "failed to decode output")?;
        return Err(anyhow!("Failed to execute rustup. Stderr - {:?}", stderr));
    }
    Ok(())
}

impl Version {
    pub fn get_string(self) -> &'static str {
        match self {
            Version::V1_34 => "1.34.0",
            Version::V1_35 => "1.35.0",
            Version::V1_36 => "1.36.0",
            Version::V1_37 => "1.37.0",
            Version::V1_38 => "1.38.0",
            Version::V1_39 => "1.39.0",
            Version::V1_40 => "1.40.0",
            Version::V1_41 => "1.41.0",
            Version::V1_42 => "1.42.0",
            Version::V1_43 => "1.43.0",
            Version::V1_44 => "1.44.0",
            Version::V1_45 => "1.45.0",
            Version::V1_46 => "1.46.0",
            Version::V1_47 => "1.47.0",
            Version::V1_48 => "1.48.0",
            Version::V1_49 => "1.49.0",
            Version::V1_50 => "1.50.0",
            Version::V1_51 => "1.51.0",
            Version::V1_52 => "1.52.0",
            Version::V1_53 => "1.53.0",
        }
    }
}

impl std::str::FromStr for Version {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1.34.0" => Ok(Version::V1_34),
            "1.35.0" => Ok(Version::V1_35),
            "1.36.0" => Ok(Version::V1_36),
            "1.37.0" => Ok(Version::V1_37),
            "1.38.0" => Ok(Version::V1_38),
            "1.39.0" => Ok(Version::V1_39),
            "1.40.0" => Ok(Version::V1_40),
            "1.41.0" => Ok(Version::V1_41),
            "1.42.0" => Ok(Version::V1_42),
            "1.43.0" => Ok(Version::V1_43),
            "1.44.0" => Ok(Version::V1_44),
            "1.45.0" => Ok(Version::V1_45),
            "1.46.0" => Ok(Version::V1_46),
            "1.47.0" => Ok(Version::V1_47),
            "1.48.0" => Ok(Version::V1_48),
            "1.49.0" => Ok(Version::V1_49),
            "1.50.0" => Ok(Version::V1_50),
            "1.51.0" => Ok(Version::V1_51),
            "1.52.0" => Ok(Version::V1_52),
            "1.53.0" => Ok(Version::V1_53),
            _ => Err("unknown version"),
        }
    }
}
