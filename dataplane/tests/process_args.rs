use dataplane::process::ProcessHandle;

#[test]
fn process_spawn_with_args_works() {
    let mut p = ProcessHandle::spawn_with_args("/bin/echo", &["hello"]).unwrap();

    p.wait().unwrap();
}
