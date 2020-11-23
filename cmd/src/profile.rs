use std::collections::BTreeMap;

use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize, de::Visitor, Serializer, Deserializer};

use crate::cargo::{Bytes, CompilerMode, Milliseconds, ProfileMode};
use crate::rustup::Version;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub(crate) struct Profile {
    compile_times: BTreeMap<CompileTimeProfileKey, Vec<Milliseconds>>,
    output_sizes: BTreeMap<SizeProfileKey, Bytes>,
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct CompileTimeProfileKey(Version, CompilerMode, ProfileMode);

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct SizeProfileKey(Version, CompilerMode);

impl Profile {
    pub(crate) fn new() -> Profile {
        Profile {
            compile_times: BTreeMap::new(),
            output_sizes: BTreeMap::new(),
        }
    }

    pub(crate) fn add_compile_times(
        self: &mut Self,
        version: Version,
        compile_times: BTreeMap<(CompilerMode, ProfileMode), Vec<Milliseconds>>,
    ) {
        for ((compiler_mode, profile_mode), timings) in compile_times {
            let key = CompileTimeProfileKey(version, compiler_mode, profile_mode);
            self.compile_times.insert(key, timings);
        }
    }

    pub(crate) fn add_output_sizes(
        self: &mut Self,
        version: Version,
        debug_size: Bytes,
        release_size: Bytes,
    ) {
        self.output_sizes
            .insert(SizeProfileKey(version, CompilerMode::Debug), debug_size);
        self.output_sizes
            .insert(SizeProfileKey(version, CompilerMode::Release), release_size);
    }

    pub(crate) fn versions_to_profile(self: &Profile, min_version: Version) -> Vec<Version> {
        Version::into_enum_iter()
            .filter(|v| *v as u8 >= min_version as u8)
            .filter(|v| !self.version_profiled(v))
            .collect()
    }

    fn version_profiled(self: &Self, version: &Version) -> bool {
        for compiler_mode in CompilerMode::into_enum_iter() {
            for profile_mode in ProfileMode::into_enum_iter() {
                let key = CompileTimeProfileKey(*version, compiler_mode, profile_mode);
                if !self.compile_times.contains_key(&key) {
                    return false;
                }
            }
        }
        if !self
            .output_sizes
            .contains_key(&SizeProfileKey(*version, CompilerMode::Debug))
            || !self
                .output_sizes
                .contains_key(&SizeProfileKey(*version, CompilerMode::Release))
        {
            return false;
        }
        return true;
    }
}

struct CKeyVisitor;

impl<'de> Visitor<'de> for CKeyVisitor {
    type Value = CompileTimeProfileKey;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a comma separated string like '1.34.0,Check,Incremental'")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let parts: Vec<&str> = value.split(",").collect();
        let version: Version = parts[0].parse().map_err(serde::de::Error::custom)?;
        let compiler_mode: CompilerMode = parts[1].parse().map_err(serde::de::Error::custom)?;
        let profile_mode: ProfileMode = parts[2].parse().map_err(serde::de::Error::custom)?;
        
        Ok(CompileTimeProfileKey(version, compiler_mode, profile_mode))
    }
}


struct SKeyVisitor;

impl<'de> Visitor<'de> for SKeyVisitor {
    type Value = SizeProfileKey;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a comma separated string like '1.34.0,Release'")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let parts: Vec<&str> = value.split(",").collect();
        let version: Version = parts[0].parse().map_err(serde::de::Error::custom)?;
        let compiler_mode: CompilerMode = parts[1].parse().map_err(serde::de::Error::custom)?;
        
        Ok(SizeProfileKey(version, compiler_mode))
    }
}


impl<'de> Deserialize<'de> for CompileTimeProfileKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CKeyVisitor)
    }
}


impl<'de> Deserialize<'de> for SizeProfileKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SKeyVisitor)
    }
}


impl Serialize for CompileTimeProfileKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let v = format!("{},{:?},{:?}", self.0.get_string(), self.1, self.2);
        serializer.serialize_str(&v)
    }
}

impl Serialize for SizeProfileKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let v = format!("{},{:?}", self.0.get_string(), self.1);
        serializer.serialize_str(&v)
    }
}
#[cfg(test)]
mod test {
    use crate::rustup::Version;
    use anyhow::Result;
    #[test]
    fn test_versions_to_profile() -> Result<()> {
        let profile: super::Profile = serde_json::from_str(
            r#"
            {
                "compile_times": {},
                "output_sizes": {}
            }"#,
        )?;
        assert_eq!(
            profile.versions_to_profile(Version::V1_43),
            vec![
                Version::V1_43,
                Version::V1_44,
                Version::V1_45,
                Version::V1_46,
                Version::V1_47,
                Version::V1_48,
            ]
        );

        let profile: super::Profile = serde_json::from_str(
            r#"
            {
                "compile_times": {
                    "1.43.0,Check,Clean" : [],
                    "1.43.0,Check,Incremental" : [],
                    "1.43.0,Check,PatchIncremental" : [],
                    "1.43.0,Debug,Clean" : [],
                    "1.43.0,Debug,Incremental" : [],
                    "1.43.0,Debug,PatchIncremental" : [],
                    "1.43.0,Release,Clean" : [],
                    "1.43.0,Release,Incremental" : [],
                    "1.43.0,Release,PatchIncremental" : []
                },
                "output_sizes": {
                    "1.43.0,Debug" : 0,
                    "1.43.0,Release" : 0
                }
            }"#,
        )?;
        assert_eq!(
            profile.versions_to_profile(Version::V1_43),
            vec![
                Version::V1_43,
                Version::V1_44,
                Version::V1_45,
                Version::V1_46,
                Version::V1_47,
                Version::V1_48,
            ]
        );

        Ok(())
    }
}
