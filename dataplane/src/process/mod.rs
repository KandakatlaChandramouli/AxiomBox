use std::process::{Child, Command};

pub struct ProcessHandle {
    child: Child,
}

impl ProcessHandle {
    pub fn spawn(cmd: &str) -> std::io::Result<Self> {
        let child = Command::new(cmd).spawn()?;
        Ok(Self { child })
    }

    pub fn spawn_with_args(cmd: &str, args: &[&str]) -> std::io::Result<Self> {
        let child = Command::new(cmd).args(args).spawn()?;

        Ok(Self { child })
    }

    pub fn id(&self) -> u32 {
        self.child.id()
    }

    pub fn wait(&mut self) -> std::io::Result<()> {
        self.child.wait()?;
        Ok(())
    }

    pub fn kill(&mut self) -> std::io::Result<()> {
        self.child.kill()
    }
}
