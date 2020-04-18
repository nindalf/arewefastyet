mod cargo;
mod repo;
mod rustup;
mod store;

use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};

fn get_clap_app<'a, 'b>() -> App<'a, 'b> {
    App::new("arewefastyet")
        .version("0.1.0")
        .author("Krishna Sundarram <krishna.sundarram@gmail.com>")
        .about("Benchmarks the rust compiler")
        .arg(
            Arg::with_name("repos")
                .long("repos")
                .default_value("data/repos.json")
                .takes_value(true)
                .help("File containing repos to clone and compile"),
        )
        .arg(
            Arg::with_name("results")
                .long("results")
                .default_value("data/results.json")
                .takes_value(true)
                .help("Results will be written to this file"),
        )
        .arg(
            Arg::with_name("dir")
                .long("working_directory")
                .default_value("/tmp/prof")
                .takes_value(true)
                .help("Directory to store the repos in"),
        )
        .arg(
            Arg::with_name("times")
                .long("times")
                .default_value("10")
                .takes_value(true)
                .help("Number of times each benchmark should be run"),
        )
}

fn get_arg<'a>(matches: &'a ArgMatches<'a>, arg: &str) -> Result<&'a str> {
    matches
        .value_of(arg)
        .ok_or_else(|| anyhow!("Could not find arg - {}", arg))
}

fn main() -> Result<()> {
    rustup::set_profile_minimal()?;

    let matches = get_clap_app().get_matches();

    let working_directory = get_arg(&matches, "dir")?;
    repo::create_working_directory(working_directory)?;

    let times = u32::from_str(get_arg(&matches, "times")?)?;

    let repo_file = get_arg(&matches, "repos")?;
    let results_file = get_arg(&matches, "results")?;
    let mut results = store::get_results(results_file, repo_file)?;

    // hack to allow writing of results after every iteration
    let repo_names: Vec<String> = results.keys().map(|s| s.to_owned()).collect();
    for repo in repo_names {
        let perf = results.get_mut(&repo).ok_or(anyhow!("impossible"))?;
        perf.repo.clone_repo()?;
        for version in perf.versions_to_profile() {
            rustup::set_version(version)?;
            let bench = cargo::benchmark(&perf.repo, times)?;
            perf.add_bench(version, bench);
        }
        store::overwrite_results(results_file, &results)?;
    }
    Ok(())
}
