use dataplane::sandbox::namespace::NamespaceGuard;

#[test]
fn namespace_creation_works() {
    match NamespaceGuard::new() {
        Ok(_) => {}
        Err(e) if e.raw_os_error() == Some(1) => {}
        Err(e) => panic!("{e:?}"),
    }
}
