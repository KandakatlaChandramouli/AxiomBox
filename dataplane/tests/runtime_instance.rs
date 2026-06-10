use dataplane::process::ProcessHandle;
use dataplane::runtime::SandboxInstance;
use dataplane::sandbox::cgroup::CgroupGuard;
use dataplane::sandbox::namespace::NamespaceGuard;

#[test]
fn runtime_creation_works() {
    let process = ProcessHandle::spawn("/bin/true").unwrap();
    let cgroup = CgroupGuard::new("runtime_test").unwrap();
    let namespace = NamespaceGuard::new().unwrap();

    let _runtime = SandboxInstance::new(process, cgroup, namespace);
}
