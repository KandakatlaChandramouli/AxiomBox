use dataplane::process::ProcessHandle;

#[test]
fn process_spawn_works() {
    let p = ProcessHandle::spawn("/bin/true").unwrap();
    assert!(p.id() > 0);
}
