use std::fs;
use std::path::PathBuf;

pub struct CgroupLimits {
    path: PathBuf,
}

impl CgroupLimits {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn set_memory_limit(&self, bytes: u64) -> std::io::Result<()> {
        fs::write(self.path.join("memory.max"), bytes.to_string())
    }

    pub fn set_pid_limit(&self, count: u64) -> std::io::Result<()> {
        fs::write(self.path.join("pids.max"), count.to_string())
    }

    pub fn set_cpu_limit(&self, quota: u64, period: u64) -> std::io::Result<()> {
        fs::write(self.path.join("cpu.max"), format!("{quota} {period}"))
    }
}
