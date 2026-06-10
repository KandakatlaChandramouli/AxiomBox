use crate::process::ProcessHandle;
use crate::runtime::SandboxInstance;
use crate::sandbox::cgroup::CgroupGuard;
use crate::sandbox::namespace::NamespaceGuard;
use crate::sandbox::resource_limits::ResourceLimits;

pub struct SandboxBuilder {
    command: String,
    limits: ResourceLimits,
}

impl SandboxBuilder {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            limits: ResourceLimits {
                memory_bytes: None,
                pid_limit: None,
                cpu_quota: None,
                cpu_period: None,
            },
        }
    }

    pub fn memory_limit(mut self, bytes: u64) -> Self {
        self.limits.memory_bytes = Some(bytes);
        self
    }

    pub fn pid_limit(mut self, count: u64) -> Self {
        self.limits.pid_limit = Some(count);
        self
    }

    pub fn cpu_limit(mut self, quota: u64, period: u64) -> Self {
        self.limits.cpu_quota = Some(quota);
        self.limits.cpu_period = Some(period);
        self
    }

    pub fn build(self) -> anyhow::Result<SandboxInstance> {
        let process = ProcessHandle::spawn(&self.command)?;
        let cgroup = CgroupGuard::new("sandbox")?;

        self.limits.apply(cgroup.path().to_path_buf())?;

        let namespace = NamespaceGuard::new()?;

        Ok(SandboxInstance::new(process, cgroup, namespace))
    }
}
