use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use once_cell::sync::OnceCell;
use once_cell::unsync::Lazy;
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

enum GitCommand {
    Checkout,
    CloneRepo,
    Reset,
}

impl Repo {
    pub(crate) fn clone_repo(self: &Repo) -> Result<()> {
        let repo_dir = self
            .get_base_directory()
            .ok_or_else(|| anyhow!("Could not find repo dir"))?;
        if !repo_dir.exists() {
            log::info!("Cloning {}", &self.name);
            self.git(GitCommand::CloneRepo)?;
            log::info!("Successfully cloned repo {}", &self.name);
        }
        self.git(GitCommand::Checkout)?;
        Ok(())
    }

    pub(crate) fn remove_target_dir(self: &Repo) -> Result<()> {
        let target_dir = self
            .get_target_directory()
            .ok_or_else(|| anyhow!("Could not find target directory"))?;
        if !target_dir.exists() {
            log::warn!("Directory {:?} doesn't exist. Skipping delete", &target_dir);
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
        let re = Lazy::new(|| {
            regex::Regex::new("((fn main.*)|(pub fn.*))").unwrap()
        });
        let contents = re.replace(&contents, r#"$1 log::info!("hello");"#);
        std::fs::write(&touch_file, contents.as_ref())
            .with_context(|| anyhow!("Failed to modify touch file - {:?}", touch_file))?;
        Ok(())
    }

    pub(crate) fn git_reset(self: &Repo) -> Result<()> {
        self.git(GitCommand::Reset)
    }

    pub(crate) fn get_base_directory(self: &Repo) -> Option<PathBuf> {
        let dir = WORKING_DIRECTORY.get()?;
        Some(dir.join(&self.name).join(&self.sub_directory))
    }

    pub(crate) fn get_debug_output_path(self: &Repo) -> Option<PathBuf> {
        self.get_output_path("debug")
    }

    pub(crate) fn get_release_output_path(self: &Repo) -> Option<PathBuf> {
        self.get_output_path("release")
    }

    fn get_target_directory(self: &Repo) -> Option<PathBuf> {
        let dir = WORKING_DIRECTORY.get()?;
        Some(dir.join(&self.name).join("target"))
    }

    fn get_output_path(self: &Repo, folder: &str) -> Option<PathBuf> {
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

    fn git(self: &Repo, command: GitCommand) -> Result<()> {
        use GitCommand::*;
        let directory = match command {
            Checkout | Reset => 
                self
                    .get_base_directory()
                    .ok_or_else(|| anyhow!("Could not find repo dir"))?,
            
            CloneRepo => 
                WORKING_DIRECTORY
                    .get()
                    .ok_or_else(|| anyhow!("Working directory not set"))?
                    .to_path_buf(),
            
        };
        let args: [&str; 2] = match command {
            Checkout => ["checkout", &self.commit_hash],
            CloneRepo => ["clone", &self.url],
            Reset => ["reset", "--hard"],
        };

        let output = Command::new("git")
            .current_dir(&directory)
            .args(&args)
            .output()
            .with_context(|| "failed to execute git")?;

        if !output.status.success() {
            let stderr = std::str::from_utf8(&output.stderr)
                .with_context(|| "failed to decode git stderr")?;
            return Err(anyhow!(
                "Failed to execute git. Dir - {:?}. Args - {:?}.\n Stderr - {}",
                &directory,
                &args,
                stderr
            ));
        }
        Ok(())
    }
}

pub(crate) fn create_working_directory(mut working_dir: PathBuf) -> Result<()> {
    WORKING_DIRECTORY.get_or_init(|| {
        if !working_dir.ends_with(ARE_WE_FAST_YET) {
            working_dir.push(ARE_WE_FAST_YET);
        }
        if !working_dir.exists() {
            log::info!("Creating {:?}", &working_dir);
            std::fs::create_dir_all(&working_dir)
                .with_context(|| "Failed to create working directory").unwrap();
        }
        log::info!("Created working directory - {:?}", working_dir);
        working_dir
    });
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_println_git_reset() -> Result<()> {
        crate::rustup::set_profile_minimal()?;
        create_working_directory(std::path::PathBuf::from("/tmp/prof/"))?;

        let repo: crate::repo::Repo = serde_json::from_str(
            r#"
            {
                "name": "helloworld",
                "sub_directory": "",
                "url": "https://github.com/nindalf/helloworld",
                "touch_file": "src/main.rs",
                "output": "helloworld",
                "commit_hash": "0ee163a",
                "min_version": "V1_45"
            }"#,
        )?;
        repo.clone_repo()?;
        let touch_file = repo.get_touch_file().unwrap();
        let initial_size = file_size(&touch_file)?;

        repo.add_println()?;
        let modified_size = file_size(&touch_file)?;

        repo.git_reset()?;
        let final_size = file_size(&touch_file)?;

        assert_eq!(initial_size + 19, modified_size);
        assert_eq!(initial_size, final_size);
        
        Ok(())
    }

    fn file_size(path: &PathBuf) -> Result<u64> {
        let file = std::fs::File::open(&path)
            .with_context(|| anyhow!("failed to find file - {:?}", path))?;
        let metadata = file.metadata()?;
        Ok(metadata.len())
    }
}
