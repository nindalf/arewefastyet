use anyhow::{anyhow, Context, Result};
use once_cell::unsync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Serialize, Deserialize)]
pub(crate) struct SystemInfo {
    pub(crate) num_cores: usize,
    num_physical_cores: usize,
    cpu_model: String,
}

impl SystemInfo {
    pub(crate) fn new() -> Result<SystemInfo> {
        let cpu_model = SystemInfo::cpu_model()?;
        Ok(SystemInfo {
            num_cores: num_cpus::get(),
            num_physical_cores: num_cpus::get_physical(),
            cpu_model,
        })
    }

    // Only works on Linux
    fn cpu_model() -> Result<String> {
        let re = Lazy::new(|| {
            regex::Regex::new("model name.*: (.*)").unwrap()
        });
        SystemInfo::read_file("/proc/cpuinfo", &re)
    }

    fn read_file(file_name: &str, re: &regex::Regex) -> Result<String> {
        let all_info: String = std::fs::read_to_string(file_name)
            .with_context(|| anyhow!("Unable to open file - {}", file_name))?;
        let m = re.captures(&all_info)
            .ok_or_else(|| anyhow!("failed to capture"))?
            .get(1)
            .ok_or_else(|| anyhow!("unable to find text"))?;
        Ok(m.as_str().to_string())
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use super::*;
    #[test]
    fn test_fill_system_info() -> Result<()> {
        println!("{:?}", SystemInfo::new()?);
        Ok(())
    }
}