use crate::checkpoint::metadata::SnapshotMetadata;

pub trait Checkpointable {
    fn checkpoint(&self) -> SnapshotMetadata;
}
