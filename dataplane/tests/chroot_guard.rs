use dataplane::sandbox::chroot::ChrootGuard;

#[test]
fn chroot_guard_creation_works() {
    let guard = ChrootGuard::new("/tmp/rootfs");

    assert_eq!(guard.rootfs().to_str().unwrap(), "/tmp/rootfs");
}
