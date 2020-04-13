mod repo;
mod command;

use crate::repo::{get_repos};
use repo::{Version};
use std::collections::HashMap;
// use crate::command;
fn main() -> Result<(), &'static str> {
    let supported_versions = vec![Version::V1_34, Version::V1_35, Version::V1_36, Version::V1_37, Version::V1_38, Version::V1_39, Version::V1_40, Version::V1_41, Version::V1_42];
    command::set_profile_minimal();
    command::create_working_directory();
    let mut results = HashMap::new();
    let repos = get_repos("src/repos.json")?;
    for repo in &repos {
        command::clone_repo(&repo);
        for version in &supported_versions {
            let bench = command::benchmark(repo, *version);
            for (mode, time) in bench {
                let entry = results.entry(mode).or_insert(vec![]);
                entry.push((version, time));
            }
        }
    }
    println!("Output - {:?}", &results);
    let result_string = serde_json::to_string(&results);
    println!("Output - {}", result_string.unwrap());
    Ok(())
}
