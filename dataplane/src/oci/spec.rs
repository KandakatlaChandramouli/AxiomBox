use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OciSpec {
    #[serde(rename = "ociVersion")]
    pub oci_version: String,

    pub process: Process,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Process {
    pub terminal: bool,

    pub args: Vec<String>,

    pub cwd: String,
}
