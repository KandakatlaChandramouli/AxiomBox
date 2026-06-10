use crate::process::ProcessHandle;
use crate::runtime::SandboxInstance;
use crate::sandbox::cgroup::CgroupGuard;
use crate::sandbox::namespace::NamespaceGuard;

pub struct SandboxBuilder {
    command: String,
}

impl SandboxBuilder {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
        }
    }

    pub fn build(self) -> anyhow::Result<SandboxInstance> {
        let process = ProcessHandle::spawn(&self.command)?;
        let cgroup = CgroupGuard::new("sandbox")?;
        let namespace = NamespaceGuard::new()?;

        Ok(SandboxInstance::new(process, cgroup, namespace))
    }
}
