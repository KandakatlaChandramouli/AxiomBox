use crate::process::ProcessHandle;
use crate::runtime::SandboxInstance;
use crate::sandbox::cgroup::CgroupGuard;
use crate::sandbox::namespace::NamespaceGuard;

pub struct SandboxBuilder {
    command: String,
    args: Vec<String>,
}

impl SandboxBuilder {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            args: Vec::new(),
        }
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn build(self) -> anyhow::Result<SandboxInstance> {
        let refs: Vec<&str> = self.args.iter().map(|s| s.as_str()).collect();

        let process = ProcessHandle::spawn_with_args(&self.command, &refs)?;

        let cgroup = CgroupGuard::new("sandbox")?;
        let namespace = NamespaceGuard::new()?;

        Ok(SandboxInstance::new(process, cgroup, namespace))
    }
}
