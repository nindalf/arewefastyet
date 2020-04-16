mod cargo;
mod repo;
mod rustup;

static TIMES: u32 = 10;

fn main() -> Result<(), &'static str> {
    rustup::set_profile_minimal();
    repo::create_working_directory();
    let repos = repo::get_repos("data/repos.json")?;
    let mut results: Vec<repo::Perf> = Vec::new();
    for repo in &repos {
        repo.clone_repo();
        let mut perf = repo::Perf::new(repo.clone());
        for version in repo.supported_versions() {
            rustup::set_version(version);
            let bench = cargo::benchmark(repo, TIMES);
            perf.add_bench(version, bench);
        }
        results.push(perf);
    }
    println!("Output - {:?}", &results);
    let output = std::fs::File::create("data/results.json").unwrap();
    serde_json::to_writer_pretty(output, &results).unwrap();
    Ok(())
}
