use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;

use anyhow::Result;

use crate::repo::{Perf, Repo};

pub(crate) fn get_results(results_file: &PathBuf, repo_file: &PathBuf) -> Result<HashMap<String, Perf>> {
    let mut perf = if results_file.exists() {
        let file = File::open(results_file)?;
        serde_json::from_reader(file).unwrap_or(HashMap::new())
    } else {
        HashMap::new()
    };
    let file = File::open(repo_file)?;
    let repos: Vec<Repo> = serde_json::from_reader(file)?;

    for repo in repos {
        perf.entry(repo.name.to_owned())
            .and_modify(|perf: &mut Perf| perf.set_repo(repo.clone()))
            .or_insert_with(|| Perf::new(repo));
    }

    Ok(perf)
}

pub(crate) fn overwrite_results(results_file: &PathBuf, results: &HashMap<String, Perf>) -> Result<()> {
    let output = File::create(results_file)?;
    serde_json::to_writer_pretty(output, results)?;
    Ok(())
}
