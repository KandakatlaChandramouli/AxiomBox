use dataplane::sandbox::namespace::NamespaceGuard;

#[test]
fn namespace_creation_works() {
    let _guard = NamespaceGuard::new().unwrap();
}
