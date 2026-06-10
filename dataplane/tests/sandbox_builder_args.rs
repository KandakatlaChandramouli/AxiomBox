use dataplane::sandbox::builder::SandboxBuilder;

#[test]
fn sandbox_builder_args_work() {
    let _sandbox = SandboxBuilder::new("/bin/echo")
        .arg("hello")
        .build()
        .unwrap();
}
