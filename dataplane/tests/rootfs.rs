use dataplane::sandbox::rootfs::RootFs;

#[test]
fn rootfs_creation_works() {
    let rootfs = RootFs::new("/tmp/rootfs".into());

    assert_eq!(rootfs.path().to_str().unwrap(), "/tmp/rootfs");
}
