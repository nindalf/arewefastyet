mod cargo;
mod repo;
mod rustup;
mod store;

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "arewefastyet", about = "Benchmark the rust compiler")]
struct Opt {
    #[structopt(short, long, default_value = "10")]
    times: u32,
    #[structopt(short, long, default_value = "/tmp/prof", parse(from_os_str))]
    working_directory: PathBuf,
    #[structopt(long, default_value = "../data/repos.json", parse(from_os_str))]
    repos: PathBuf,
    #[structopt(long, default_value = "../data/results.json", parse(from_os_str))]
    results: PathBuf,
}

fn main() -> Result<()> {
    rustup::set_profile_minimal()?;

    let opt = Opt::from_args();
    repo::create_working_directory(opt.working_directory)?;

    let mut results = store::get_results(&opt.results, &opt.repos)?;

    // hack to allow writing of results after every iteration
    let repo_names: Vec<String> = results.keys().map(|s| s.to_owned()).collect();
    for repo in repo_names {
        let profile = results.get_mut(&repo).ok_or(anyhow!("impossible"))?;
        profile.repo.clone_repo()?;
        for version in profile.versions_to_profile() {
            rustup::set_version(version)?;
            match cargo::benchmark(&profile.repo, opt.times) {
                Ok((bench, debug_size, release_size)) => {
                    profile.add_bench(version, bench, debug_size, release_size)
                }
                Err(_) => {}
            };
        }
        store::overwrite_results(&opt.results, &results)?;
    }
    Ok(())
}
