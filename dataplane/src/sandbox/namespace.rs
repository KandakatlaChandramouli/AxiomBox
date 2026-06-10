pub struct NamespaceGuard;

impl NamespaceGuard {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(Self)
    }
}
