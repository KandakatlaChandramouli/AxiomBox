use nix::errno::Errno;
use nix::sched::{CloneFlags, unshare};

pub struct NamespaceGuard;

impl NamespaceGuard {
    pub fn new() -> Result<Self, nix::Error> {
        match unshare(CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWUTS) {
            Ok(_) => Ok(Self),

            Err(Errno::EPERM) => {
                eprintln!("namespace isolation unavailable");
                Ok(Self)
            }

            Err(e) => Err(e),
        }
    }
}
