use dataplane::process::ProcessHandle;

#[test]
fn process_spawn_and_wait() {
    let mut p = ProcessHandle::spawn("/bin/true").unwrap();
    p.wait().unwrap();
}

#[test]
fn process_spawn_and_kill() {
    let mut p = ProcessHandle::spawn("sleep").unwrap_or_else(|_| {
        panic!("sleep not found");
    });

    let _ = p.kill();
}
