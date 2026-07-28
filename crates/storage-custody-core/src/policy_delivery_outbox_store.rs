use std::{
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
};

use atomicwrites::{AllowOverwrite, AtomicFile};
use fs2::FileExt;
use serde::{Deserialize, Serialize};

const STORE_VERSION: u16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDeliveryIntentState {
    Queued,
    Delivered,
    Degraded,
    Offline,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDeliveryOutboxIntent {
    pub version: u16,
    pub delivery_id: String,
    pub policy_decision_ref: String,
    pub target_ref: String,
    pub sequence: u64,
    pub state: PolicyDeliveryIntentState,
}

pub struct PolicyDeliveryOutboxStore {
    path: PathBuf,
}

impl PolicyDeliveryOutboxStore {
    pub fn open(directory: impl AsRef<Path>) -> io::Result<Self> {
        fs::create_dir_all(directory.as_ref())?;
        if fs::symlink_metadata(directory.as_ref())?
            .file_type()
            .is_symlink()
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "custody store directory must not be a symlink",
            ));
        }
        let directory = directory.as_ref().canonicalize()?;
        Ok(Self {
            path: directory.join("policy-delivery-outbox.json"),
        })
    }

    pub fn intents(&self) -> io::Result<Vec<PolicyDeliveryOutboxIntent>> {
        match fs::read(&self.path) {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(error) => Err(error),
        }
    }

    pub fn persist_queued_intent(
        &self,
        delivery_id: String,
        policy_decision_ref: String,
        target_ref: String,
        sequence: u64,
    ) -> io::Result<()> {
        let lock = self.lock()?;
        lock.lock_exclusive()?;
        let mut intents = self.intents()?;
        let candidate = PolicyDeliveryOutboxIntent {
            version: STORE_VERSION,
            delivery_id,
            policy_decision_ref,
            target_ref,
            sequence,
            state: PolicyDeliveryIntentState::Queued,
        };
        let result = persist_intent(&mut intents, candidate).and_then(|changed| {
            if changed {
                self.write(&intents)
            } else {
                Ok(())
            }
        });
        FileExt::unlock(&lock)?;
        result
    }

    pub fn mark_delivery_state(
        &self,
        delivery_id: &str,
        state: PolicyDeliveryIntentState,
    ) -> io::Result<()> {
        let lock = self.lock()?;
        lock.lock_exclusive()?;
        let mut intents = self.intents()?;
        let result = mark_state(&mut intents, delivery_id, state).and_then(|changed| {
            if changed {
                self.write(&intents)
            } else {
                Ok(())
            }
        });
        FileExt::unlock(&lock)?;
        result
    }

    fn write(&self, intents: &[PolicyDeliveryOutboxIntent]) -> io::Result<()> {
        AtomicFile::new(&self.path, AllowOverwrite)
            .write(|file| {
                serde_json::to_writer(&mut *file, intents).map_err(io::Error::other)?;
                file.sync_all()
            })
            .map_err(|error| io::Error::other(error.to_string()))?;
        sync_parent_directory(&self.path)
    }

    fn lock(&self) -> io::Result<std::fs::File> {
        OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(self.path.with_extension("lock"))
    }
}

fn persist_intent(
    intents: &mut Vec<PolicyDeliveryOutboxIntent>,
    candidate: PolicyDeliveryOutboxIntent,
) -> io::Result<bool> {
    match intents
        .iter()
        .find(|intent| intent.delivery_id == candidate.delivery_id)
    {
        Some(existing) if same_delivery_identity(existing, &candidate) => Ok(false),
        Some(_) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "policy delivery identifier conflicts with stored intent",
        )),
        None => {
            intents.push(candidate);
            Ok(true)
        }
    }
}

fn mark_state(
    intents: &mut [PolicyDeliveryOutboxIntent],
    delivery_id: &str,
    state: PolicyDeliveryIntentState,
) -> io::Result<bool> {
    let intent = intents
        .iter_mut()
        .find(|intent| intent.delivery_id == delivery_id)
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "policy delivery intent is absent")
        })?;
    if intent.state == state {
        return Ok(false);
    }
    intent.state = state;
    Ok(true)
}

fn same_delivery_identity(
    existing: &PolicyDeliveryOutboxIntent,
    candidate: &PolicyDeliveryOutboxIntent,
) -> bool {
    existing.version == candidate.version
        && existing.delivery_id == candidate.delivery_id
        && existing.policy_decision_ref == candidate.policy_decision_ref
        && existing.target_ref == candidate.target_ref
        && existing.sequence == candidate.sequence
}

#[cfg(not(windows))]
fn sync_parent_directory(path: &Path) -> io::Result<()> {
    std::fs::File::open(path.parent().unwrap_or_else(|| Path::new(".")))?.sync_all()
}

#[cfg(windows)]
fn sync_parent_directory(_path: &Path) -> io::Result<()> {
    Ok(())
}
