use anyhow::{Context, Result};

use std::fs;
use std::path::{Path, PathBuf};

pub struct CgroupGuard {
    path: PathBuf,
}

impl CgroupGuard {
    pub fn new(name: &str) -> Result<Self> {
        let path = Path::new("/sys/fs/cgroup").join(name);

        fs::create_dir(&path)
            .with_context(|| format!("failed creating cgroup {}", path.display()))?;

        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for CgroupGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir(&self.path);
    }
}
