use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::rustup::Version;

static ARE_WE_FAST_YET: &'static str = "arewefastyet-dir";
static WORKING_DIRECTORY: OnceCell<PathBuf> = OnceCell::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Repo {
    pub name: String,
    sub_directory: String,
    url: String,
    touch_file: String,
    output: String,
    commit_hash: String,
    pub min_version: Version,
}

impl Repo {
    pub(crate) fn clone_repo(self: &Repo) -> Result<()> {
        let working_directory = WORKING_DIRECTORY
            .get()
            .ok_or_else(|| anyhow!("Working directory not set"))?;
        let repo_dir = self
            .get_base_directory()
            .ok_or_else(|| anyhow!("Could not find repo dir"))?;
        if !repo_dir.exists() {
            println!("Cloning {}", &self.name);
            git(&["clone", &self.url], working_directory)?;
            println!("Successfully cloned repo {}", &self.name);
        }
        git(&["checkout", &self.commit_hash], &repo_dir)?;
        Ok(())
    }

    pub(crate) fn remove_target_dir(self: &Repo) -> Result<()> {
        let target_dir = self
            .get_target_directory()
            .ok_or_else(|| anyhow!("Could not find target directory"))?;
        if !target_dir.exists() {
            println!("Directory {:?} doesn't exist. Skipping delete", &target_dir);
            return Ok(());
        }
        std::fs::remove_dir_all(&target_dir)
            .with_context(|| anyhow!("failed to remove target directory - {:?}", target_dir))
    }

    pub(crate) fn touch_src(self: &Repo) -> Result<()> {
        let touch_file = self
            .get_touch_file()
            .ok_or_else(|| anyhow!("Could not find touch file"))?;
        if !touch_file.exists() {
            return Err(anyhow!("Touch file does not exist"));
        }
        let output = Command::new("touch")
            .args(&[&touch_file])
            .output()
            .with_context(|| "failed to execute touch")?;
        if !output.status.success() {
            let stderr = std::str::from_utf8(&output.stderr)
                .with_context(|| "failed to decode touch stderr")?;
            return Err(anyhow!(
                "Failed to touch file. Repo - {}, touch file - {:?}.\n Stderr - {}",
                &self.name,
                touch_file,
                stderr
            ));
        }
        Ok(())
    }

    pub(crate) fn add_println(self: &Repo) -> Result<()> {
        let touch_file = self
            .get_touch_file()
            .ok_or_else(|| anyhow!("Could not find touch file"))?;
        if !touch_file.exists() {
            return Err(anyhow!("Touch file does not exist"));
        }
        let contents = std::fs::read_to_string(&touch_file)
            .with_context(|| anyhow!("Failed to read touch file - {:?}"))?;
        let re = regex::Regex::new("((fn main.*)|(pub fn.*))").unwrap();
        let contents = re.replace(&contents, r#"$1 println!("hello");"#);
        std::fs::write(&touch_file, contents.as_ref())
            .with_context(|| anyhow!("Failed to modify touch file - {:?}", touch_file))?;
        Ok(())
    }

    pub(crate) fn git_reset(self: &Repo) -> Result<()> {
        let repo_dir = self
            .get_base_directory()
            .ok_or_else(|| anyhow!("Could not find repo dir"))?;
        git(&["reset", "--hard"], &repo_dir)?;
        Ok(())
    }

    pub(crate) fn get_base_directory(self: &Repo) -> Option<PathBuf> {
        let dir = WORKING_DIRECTORY.get()?;
        Some(dir.join(&self.name).join(&self.sub_directory))
    }

    pub(crate) fn get_debug_binary_path(self: &Repo) -> Option<PathBuf> {
        self.get_binary_path("debug")
    }

    pub(crate) fn get_release_binary_path(self: &Repo) -> Option<PathBuf> {
        self.get_binary_path("release")
    }

    fn get_target_directory(self: &Repo) -> Option<PathBuf> {
        let dir = WORKING_DIRECTORY.get()?;
        Some(dir.join(&self.name).join("target"))
    }

    fn get_binary_path(self: &Repo, folder: &str) -> Option<PathBuf> {
        let target_directory = self.get_target_directory()?;
        Some(target_directory.join(folder).join(&self.output))
    }

    fn get_touch_file(self: &Repo) -> Option<PathBuf> {
        let dir = WORKING_DIRECTORY.get()?;
        Some(
            dir.join(&self.name)
                .join(&self.sub_directory)
                .join(&self.touch_file),
        )
    }
}

pub(crate) fn create_working_directory(mut working_dir: PathBuf) -> Result<()> {
    if !working_dir.ends_with(ARE_WE_FAST_YET) {
        working_dir.push(ARE_WE_FAST_YET);
    }
    if !working_dir.exists() {
        println!("Creating {:?}", &working_dir);
        std::fs::create_dir_all(&working_dir)
            .with_context(|| "Failed to create working directory")?;
    }
    println!("Created working directory - {:?}", working_dir);
    WORKING_DIRECTORY
        .set(working_dir)
        .map_err(|_| anyhow!("Failed to set global variable"))?;
    Ok(())
}

fn git(args: &[&str], dir: &PathBuf) -> Result<()> {
    let output = Command::new("git")
        .current_dir(&dir)
        .args(args)
        .output()
        .with_context(|| "failed to execute git")?;
    if !output.status.success() {
        let stderr =
            std::str::from_utf8(&output.stderr).with_context(|| "failed to decode git stderr")?;
        return Err(anyhow!(
            "Failed to execute git. Args - {:?}.\n Stderr - {}",
            &args,
            stderr
        ));
    }
    Ok(())
}
