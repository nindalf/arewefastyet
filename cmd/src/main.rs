mod cargo;
mod profile;
mod repo;
mod rustup;
mod store;
mod system;

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
    repos_file: PathBuf,
    #[structopt(long, default_value = "../data/", parse(from_os_str))]
    results_dir: PathBuf,
}

fn main() -> Result<()> {
    rustup::set_profile_minimal()?;

    let opt = Opt::from_args();
    repo::create_working_directory(opt.working_directory)?;

    let mut profiles = store::get_profiles(&opt.results_dir, &opt.repos_file)?;

    // hack to allow writing of results after every iteration
    let repo_names: Vec<String> = profiles.keys().map(|s| s.to_owned()).collect();
    for repo in repo_names {
        let profile = profiles.get_mut(&repo).ok_or(anyhow!("impossible"))?;
        profile.repo.clone_repo()?;

        for version in profile.versions_to_profile() {
            rustup::set_version(version)?;

            match cargo::compile_time_profile(&profile.repo, opt.times) {
                Ok(compile_time_profile) => {
                    profile.add_compile_times(version, compile_time_profile)
                }
                Err(_) => {}
            };

            match cargo::size_profile(&profile.repo) {
                Ok((debug_size, release_size)) => {
                    profile.add_output_sizes(version, debug_size, release_size)
                }
                Err(_) => {}
            };
        }

        store::overwrite_profiles(&opt.results_dir, &profiles)?;
    }
    Ok(())
}
