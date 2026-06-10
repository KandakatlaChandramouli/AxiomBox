use std::path::PathBuf;

pub struct RootFs {
    path: PathBuf,
}

impl RootFs {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &std::path::Path {
        &self.path
    }
}
