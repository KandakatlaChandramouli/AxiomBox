use nix::sched::{CloneFlags, unshare};

pub struct NamespaceGuard {
    enabled: bool,
}

impl NamespaceGuard {
    pub fn new() -> Result<Self, std::io::Error> {
        match unshare(CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWIPC)
        {
            Ok(_) => Ok(Self { enabled: true }),

            Err(nix::errno::Errno::EPERM) => Ok(Self { enabled: false }),

            Err(e) => Err(std::io::Error::other(e)),
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}
