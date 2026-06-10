use std::path::PathBuf;

use crate::sandbox::cgroup_limits::CgroupLimits;

pub struct ResourceLimits {
    pub memory_bytes: Option<u64>,
    pub pid_limit: Option<u64>,
    pub cpu_quota: Option<u64>,
    pub cpu_period: Option<u64>,
}

impl ResourceLimits {
    pub fn apply(&self, cgroup_path: PathBuf) -> std::io::Result<()> {
        let limits = CgroupLimits::new(cgroup_path);

        if let Some(memory) = self.memory_bytes {
            limits.set_memory_limit(memory)?;
        }

        if let Some(pids) = self.pid_limit {
            limits.set_pid_limit(pids)?;
        }

        if let (Some(quota), Some(period)) = (self.cpu_quota, self.cpu_period) {
            limits.set_cpu_limit(quota, period)?;
        }

        Ok(())
    }
}
