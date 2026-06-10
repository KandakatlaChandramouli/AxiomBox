use dataplane::sandbox::namespace::NamespaceGuard;

#[test]
fn namespace_creation_works() {
    let _ns = NamespaceGuard::new().unwrap();
}
