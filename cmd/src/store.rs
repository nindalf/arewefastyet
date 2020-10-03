use std::borrow::Cow;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::profile::Profile;
use crate::repo::Repo;
use crate::system::SystemInfo;

#[derive(Debug, Serialize, Deserialize)]
struct FinalResult<'a> {
    system_info: SystemInfo,
    profiles: Cow<'a, HashMap<String, Profile>>,
}

impl<'a> FinalResult<'a> {
    fn new(system_info: SystemInfo) -> Self {
        FinalResult {
            system_info,
            profiles: Cow::Owned(HashMap::new()),
        }
    }
}

pub(crate) fn get_profiles(
    results_dir: &PathBuf,
    repos_file: &PathBuf,
) -> Result<HashMap<String, Profile>> {
    let system_info = SystemInfo::new()?;
    let results_file = get_result_file_path(results_dir, &system_info);
    let mut profiles = if results_file.exists() {
        let file = File::open(results_file)?;
        let final_result = serde_json::from_reader(file).unwrap_or(FinalResult::new(system_info));
        final_result.profiles.into_owned()
    } else {
        HashMap::new()
    };
    let file = File::open(repos_file)?;
    let repos: Vec<Repo> = serde_json::from_reader(file)?;

    for repo in repos {
        profiles
            .entry(repo.name.to_owned())
            .and_modify(|profile: &mut Profile| profile.set_repo(repo.clone()))
            .or_insert_with(|| Profile::new(repo));
    }

    Ok(profiles)
}

pub(crate) fn overwrite_profiles(
    results_dir: &PathBuf,
    profiles: &HashMap<String, Profile>,
) -> Result<()> {
    let system_info = SystemInfo::new()?;
    let results_file = get_result_file_path(results_dir, &system_info);

    let final_result = FinalResult {
        system_info,
        profiles: Cow::Borrowed(&profiles),
    };
    let output = File::create(&results_file)?;
    log::info!("Writing to {:?}", &results_file);
    serde_json::to_writer_pretty(output, &final_result)?;
    Ok(())
}

fn get_result_file_path(path: &PathBuf, system_info: &SystemInfo) -> PathBuf {
    let mut hasher = DefaultHasher::new();
    system_info.hash(&mut hasher);
    let hash = hasher.finish();
    let file_name = format!("results-{}-{}.json", system_info.num_cores, hash);
    let mut filepath = path.to_owned();
    filepath.push(file_name);
    filepath
}

#[cfg(test)]
mod test {
    use anyhow::{anyhow, Result};
    use std::path::PathBuf;
    #[test]
    fn test_file_path() -> Result<()> {
        let system_info = serde_json::from_str(
            r#"{ 
            "num_cores": 8, 
            "num_physical_cores": 4, 
            "cpu_model": "Intel(R) Core(TM) i7-8550U CPU @ 1.80GHz", 
            "total_memory": "12963196 kB" 
        }"#,
        )?;
        let file_path = super::get_result_file_path(&PathBuf::from("/tmp/"), &system_info);
        assert_eq!(
            file_path.to_str().ok_or(anyhow!("no path"))?,
            "/tmp/results-8-11383570585417069832.json"
        );
        Ok(())
    }
}
