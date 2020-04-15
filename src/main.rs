mod command;
mod repo;

fn main() -> Result<(), &'static str> {
    command::set_profile_minimal();
    command::create_working_directory();
    let repos = repo::get_repos("data/repos.json")?;
    let mut results: Vec<repo::Perf> = Vec::new();
    for repo in &repos {
        command::clone_repo(repo);
        let mut perf = repo::Perf::new(repo.clone());
        for version in repo.supported_versions() {
            let bench = command::benchmark(repo, version);
            perf.add_bench(version, bench);
        }
        results.push(perf);
    }
    println!("Output - {:?}", &results);
    let output = std::fs::File::create("data/results.json").unwrap();
    serde_json::to_writer_pretty(output, &results).unwrap();
    Ok(())
}
