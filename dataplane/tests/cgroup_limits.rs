use dataplane::sandbox::cgroup_limits::CgroupLimits;

#[test]
fn limits_object_creation_works() {
    let tmp = std::env::temp_dir().join("cg_limits_test");
    std::fs::create_dir_all(&tmp).unwrap();

    let limits = CgroupLimits::new(tmp);

    limits.set_memory_limit(1024).unwrap_or(());

    let _ = limits;
}
