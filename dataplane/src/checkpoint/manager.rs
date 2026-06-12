use std::fs;
use std::path::PathBuf;

pub struct CheckpointManager {
    root: PathBuf,
}

impl CheckpointManager {
    pub fn new() -> Self {
        Self {
            root: PathBuf::from(".dataplane/checkpoints"),
        }
    }

    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.root)
    }
}
