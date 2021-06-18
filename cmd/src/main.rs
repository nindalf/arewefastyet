mod cargo;
mod profile;
mod repo;
mod rustup;
mod store;
mod system;

use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "arewefastyet", about = "Benchmark the rust compiler")]
struct Opt {
    #[structopt(short, long, default_value = "5")]
    times: u32,
    #[structopt(short, long, default_value = "/tmp/prof", parse(from_os_str))]
    working_directory: PathBuf,
    #[structopt(long, default_value = "../data/repos.json", parse(from_os_str))]
    repos_file: PathBuf,
    #[structopt(long, default_value = "../data/", parse(from_os_str))]
    results_dir: PathBuf,
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    match exec() {
        Ok(_) => log::info!("Completed successfully"),
        Err(e) => log::error!("Error executing process - {}", e),
    };

    Ok(())
}

fn exec() -> Result<()> {
    rustup::set_profile_minimal()?;

    let opt = Opt::from_args();
    repo::create_working_directory(opt.working_directory)?;

    let repos = store::get_repos(&opt.repos_file)?;
    let mut profiles = store::get_profiles(&opt.results_dir)?;

    for repo in repos {
        repo.clone_repo()?;

        let profile = profiles
            .entry(repo.name.clone())
            .or_insert_with(profile::Profile::new);

        for version in profile.versions_to_profile(repo.min_version) {
            rustup::set_version(version)?;

            match cargo::compile_time_profile(&repo, opt.times) {
                Ok(compile_time_profile) => {
                    profile.add_compile_times(version, compile_time_profile)
                }
                Err(e) => {
                    log::error!(
                        "Failed to profile times {} on version {}. Error - {}",
                        &repo.name,
                        version.get_string(),
                        e
                    );
                }
            };

            match cargo::size_profile(&repo) {
                Ok((debug_size, release_size)) => {
                    profile.add_output_sizes(version, debug_size, release_size)
                }
                Err(e) => {
                    log::error!(
                        "Failed to profile sizes {} on version {}. Error - {}",
                        &repo.name,
                        version.get_string(),
                        e
                    );
                }
            };
        }

        store::overwrite_profiles(&opt.results_dir, &profiles)?;
    }
    Ok(())
}
