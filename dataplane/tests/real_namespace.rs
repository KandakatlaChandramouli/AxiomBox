use dataplane::sandbox::namespace::NamespaceGuard;

#[test]
fn namespace_creation_works() {
    let _ = NamespaceGuard::new();
}
