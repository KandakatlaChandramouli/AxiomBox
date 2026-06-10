use nix::sched::{CloneFlags, unshare};

pub struct NamespaceGuard;

impl NamespaceGuard {
    pub fn new() -> Result<Self, std::io::Error> {
        unshare(CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWIPC)
            .map_err(std::io::Error::other)?;

        Ok(Self)
    }
}
