use std::path::{Path, PathBuf};

pub struct ChrootGuard {
    rootfs: PathBuf,
}

impl ChrootGuard {
    pub fn new(rootfs: impl Into<PathBuf>) -> Self {
        Self {
            rootfs: rootfs.into(),
        }
    }

    pub fn rootfs(&self) -> &Path {
        &self.rootfs
    }
}
