use std::path::{Path, PathBuf};

use super::*;

pub(super) struct MetadataPaths {
    pub(super) bridge: PathBuf,
    pub(super) owner: PathBuf,
    pub(super) receipts: PathBuf,
    pub(super) intents: PathBuf,
}

pub(super) fn from_root(root: &Path) -> MetadataPaths {
    let bridge = root.join(BRIDGE_DIRECTORY);
    let owner = bridge.join(MUTATION_OWNER_DIRECTORY);
    let receipts = owner.join(RECEIPTS_DIRECTORY);
    let intents = owner.join(INTENTS_DIRECTORY);
    MetadataPaths {
        bridge,
        owner,
        receipts,
        intents,
    }
}
