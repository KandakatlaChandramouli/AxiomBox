use dataplane::sandbox::cgroup::CgroupGuard;

use std::path::Path;

#[test]
fn cgroup_created_and_removed() {

    let path =
        "/sys/fs/cgroup/axiom_test";

    {
        let guard =
            CgroupGuard::new(
                "axiom_test"
            )
            .unwrap();

        assert!(
            guard.path().exists()
        );
    }

    assert!(
        !Path::new(path).exists()
    );
}