use std::collections::HashMap;

use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};

use crate::cargo::{CompilerMode, ProfileMode, Bytes, Milliseconds};
use crate::repo::Repo;
use crate::rustup::Version;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Profile {
    pub repo: Repo,
    compile_times: HashMap<CompilerMode, HashMap<ProfileMode, CompileTimeProfile>>,
    output_sizes: HashMap<CompilerMode, SizeProfile>,
}

type CompileTimeProfile = HashMap<Version, Vec<Milliseconds>>;

type SizeProfile = HashMap<Version, Bytes>;

impl Profile {
    pub(crate) fn new(repo: Repo) -> Profile {
        Profile {
            repo,
            compile_times: HashMap::with_capacity(3),
            output_sizes: HashMap::with_capacity(2),
        }
    }

    pub(crate) fn add_compile_times(
        self: &mut Self,
        version: Version,
        compile_times: HashMap<CompilerMode, HashMap<ProfileMode, Vec<Milliseconds>>>,
    ) {
        for (compiler_mode, profile_mode_times) in compile_times {
            for (profile_mode, times) in profile_mode_times {
                self.compile_times
                    .entry(compiler_mode)
                    .or_insert(HashMap::with_capacity(3))
                    .entry(profile_mode)
                    .or_insert(HashMap::with_capacity(20))
                    .insert(version, times);
            }
        }
    }

    pub(crate) fn add_output_sizes(
        self: &mut Self,
        version: Version,
        debug_size: Bytes,
        release_size: Bytes,
    ) {
        self.output_sizes
            .entry(CompilerMode::Debug)
            .or_insert(HashMap::with_capacity(20))
            .insert(version, debug_size);
        self.output_sizes
            .entry(CompilerMode::Release)
            .or_insert(HashMap::with_capacity(20))
            .insert(version, release_size);
    }

    pub(crate) fn versions_to_profile(self: &Profile) -> Vec<Version> {
        let min = self.repo.min_version as u8;
        Version::into_enum_iter()
            .filter(|v| *v as u8 >= min)
            .filter(|v| !self.version_profiled(v))
            .collect()
    }

    fn version_profiled(self: &Self, version: &Version) -> bool {
        for compiler_mode in CompilerMode::into_enum_iter() {
            for profile_mode in ProfileMode::into_enum_iter() {
                if !self.compile_times.contains_key(&compiler_mode) {
                    return false;
                }
                let inner = self.compile_times.get(&compiler_mode).unwrap();
                if !inner.contains_key(&profile_mode) {
                    return false;
                }
                let inner = inner.get(&profile_mode).unwrap();
                if !inner.contains_key(&version) {
                    return false;
                }
            }
        }
        // TODO also check if output sizes exist
        return true;
    }

    pub(crate) fn set_repo(self: &mut Self, repo: Repo) {
        self.repo = repo;
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
                "repo": {
                    "name": "helloworld",
                    "sub_directory": "",
                    "url": "https://github.com/nindalf/helloworld",
                    "touch_file": "src/main.rs",
                    "output": "helloworld",
                    "commit_hash": "0ee163a",
                    "min_version": "V1_43"
                },
                "compile_times": {},
                "output_sizes": {}
            }"#,
        )?;
        assert_eq!(
            profile.versions_to_profile(),
            vec![
                Version::V1_43,
                Version::V1_44,
                Version::V1_45,
                Version::V1_46
            ]
        );

        let profile: super::Profile = serde_json::from_str(
            r#"
            {
                "repo": {
                    "name": "helloworld",
                    "sub_directory": "",
                    "url": "https://github.com/nindalf/helloworld",
                    "touch_file": "src/main.rs",
                    "output": "helloworld",
                    "commit_hash": "0ee163a",
                    "min_version": "V1_43"
                },
                "compile_times": {
                    "Check": {
                        "Clean": {"V1_43": []},
                        "Incremental": {"V1_43": []},
                        "PrintIncremental": {"V1_43": []}
                    },
                    "Debug": {
                        "Clean": {"V1_43": []},
                        "Incremental": {"V1_43": []},
                        "PrintIncremental": {"V1_43": []}},
                    "Release": {
                        "Clean": {"V1_43": []},
                        "Incremental": {"V1_43": []},
                        "PrintIncremental": {"V1_43": []}}
                },
                "output_sizes": {}
            }"#,
        )?;
        assert_eq!(
            profile.versions_to_profile(),
            vec![Version::V1_44, Version::V1_45, Version::V1_46]
        );

        Ok(())
    }
}
