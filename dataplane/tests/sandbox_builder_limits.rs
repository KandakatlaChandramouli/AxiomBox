use dataplane::sandbox::builder::SandboxBuilder;

#[test]
fn builder_accepts_limits() {
    let _builder = SandboxBuilder::new("sleep 1")
        .memory_limit(1024 * 1024)
        .pid_limit(32)
        .cpu_limit(50000, 100000);
}
