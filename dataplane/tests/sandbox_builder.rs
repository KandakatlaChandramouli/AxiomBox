use dataplane::sandbox::builder::SandboxBuilder;

#[test]
fn sandbox_builder_works() {
    let _sandbox = SandboxBuilder::new("/bin/true").build().unwrap();
}
