use crate::process::ProcessHandle;
use crate::sandbox::cgroup::CgroupGuard;
use crate::sandbox::namespace::NamespaceGuard;

pub struct SandboxInstance {
    pub process: ProcessHandle,
    pub cgroup: CgroupGuard,
    pub namespace: NamespaceGuard,
}

impl SandboxInstance {
    pub fn new(process: ProcessHandle, cgroup: CgroupGuard, namespace: NamespaceGuard) -> Self {
        Self {
            process,
            cgroup,
            namespace,
        }
    }
}
