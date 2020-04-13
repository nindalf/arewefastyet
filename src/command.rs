
use std::process::Command;
use std::collections::HashMap;
use crate::repo::{Version, Repo, Mode};

pub(crate) fn create_working_directory() {
    println!("Creating working directory");
    let home = dirs::home_dir().unwrap();
    let working_dir = home.join("arewefast-workdir");
        
    let res = std::fs::create_dir(working_dir);
    match res {
        Ok(_) => {
            println!("Successfully created working directory");
        },
        Err(err) => {
            if err.kind() != std::io::ErrorKind::AlreadyExists {
                panic!("Failed to create directory");
            } 
        }
    }
}

pub(crate) fn clone_repo(repo: &Repo) {
    let path = repo.get_base_directory();
    if std::path::Path::new(&path).exists() {
        println!("Directory {} already exists. Skipping clone", &path);
    } else {
        println!("Cloning {}", &repo.name);
        let output = Command::new("git")
            .current_dir("/home/nindalf/arewefast-workdir")
            .args(&["clone", &repo.url])
            .output()
            .expect("failed to git clone");
        if output.status.success() {
            println!("Successfully cloned repo {}", &repo.name);
        } else {
            let stderr = std::str::from_utf8(&output.stderr).expect("failed to decode output");
            println!("Failed to clone repo {}. Stderr - {}", &repo.name, stderr);
        }
    }
    
    Command::new("git")
        .current_dir(&path)
        .args(&["checkout", &repo.commit_hash])
        .output()
        .expect("failed to git checkout");
}

pub(crate) fn benchmark(repo: &Repo, version: Version) ->  HashMap<Mode, String> {
    download_toolchain(version);
    switch_toolchain(version);
    cargo_check(repo); // download dependencies
    remove_target_folder(repo);
    let mut results = HashMap::new();

    let check_first = cargo_check(repo);
    results.insert(Mode::Check, check_first);
    touch_src(repo);
    let check_incremental = cargo_check(repo);
    results.insert(Mode::CheckIncremental, check_incremental);

    let debug_first = cargo_debug(repo);
    results.insert(Mode::Debug, debug_first);
    touch_src(repo);
    let debug_incremental = cargo_debug(repo);
    results.insert(Mode::DebugIncremental, debug_incremental);

    let release_first = cargo_release(repo);
    results.insert(Mode::Release, release_first);
    touch_src(repo);
    let release_incremental = cargo_release(repo);
    results.insert(Mode::ReleaseIncremental, release_incremental);

    results
}

fn cargo_check(repo: &Repo) -> String {
    println!("{} - Running cargo check", &repo.name);
    let dir = repo.get_base_directory();
    let output = Command::new("cargo")
        .current_dir(dir)
        .args(&["check"])
        .output()
        .expect("failed to execute cargo");
    let stderr = std::str::from_utf8(&output.stderr).expect("failed to decode output");
    parse_run_time(stderr)
}

fn cargo_debug(repo: &Repo) -> String {
    println!("{} - Running cargo build", &repo.name);
    let dir = repo.get_base_directory();
    let output = Command::new("cargo")
        .current_dir(dir)
        .args(&["build"])
        .output()
        .expect("failed to execute cargo");
    let stderr = std::str::from_utf8(&output.stderr).expect("failed to decode output");
    parse_run_time(stderr)
}

fn cargo_release(repo: &Repo) -> String {
    println!("{} - Running cargo release", &repo.name);
    let dir = repo.get_base_directory();
    let output = Command::new("cargo")
        .current_dir(dir)
        .args(&["build", "--release"])
        .output()
        .expect("failed to execute cargo");
    let stderr = std::str::from_utf8(&output.stderr).expect("failed to decode output");
    parse_run_time(stderr)
}

fn parse_run_time(stderr: &str) -> String {
    let line = stderr.lines().last().unwrap();
    line.split("in ").last().unwrap().to_string()
}

fn remove_target_folder(repo: &Repo) {
    let path = repo.get_target_directory();
    if !std::path::Path::new(&path).exists() {
        println!("Directory {} doesn't exist. Skipping delete", &path);
    }
    std::fs::remove_dir_all(path).unwrap();
}

fn touch_src(repo: &Repo) {
    Command::new("touch")
        .args(&[repo.get_touch_file()])
        .output()
        .expect("failed to execute touch");
}

fn download_toolchain(version: Version) {
    let version = version.get_string();
    println!("Downloading toolchain {}", version);
    let output = Command::new("rustup")
        .args(&["toolchain", "install", version])
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        println!("Successfully downloaded toolchain {}.", version);
    } else {
        let stderr = std::str::from_utf8(&output.stderr).expect("failed to decode output");
        println!("Failed to download toolchain {}. Stderr - {}", version, stderr);
    }
}

fn switch_toolchain(version: Version) {
    let version = Version::get_string(version);
    println!("Switching toolchain to {}", version);
    let output = Command::new("rustup")
        .args(&["default", version])
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        println!("Successfully switched toolchain {}.", version);
    } else {
        let stderr = std::str::from_utf8(&output.stderr).expect("failed to decode output");
        println!("Failed to switch toolchain {}. Stderr - {}", version, stderr);
    }
}

pub(crate) fn set_profile_minimal() {
    println!("Setting profile to minimal");
    let output = Command::new("rustup")
        .args(&["set", "profile", "minimal"])
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        println!("Successfully set profile");
    } else {
        println!("Failed to set profile");
    }
}