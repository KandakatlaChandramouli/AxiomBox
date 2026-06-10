use dataplane::sandbox::cleanup::CleanupManager;

use std::sync::{
    Arc,
    atomic::{
        AtomicBool,
        Ordering
    }
};

#[test]
fn cleanup_runs() {

    let flag =
        Arc::new(
            AtomicBool::new(false)
        );

    {
        let cleanup =
            CleanupManager::new();

        let local =
            flag.clone();

        cleanup.register(move || {
            local.store(
                true,
                Ordering::SeqCst,
            );
        });
    }

    assert!(
        flag.load(
            Ordering::SeqCst
        )
    );
}