//! Windows DPAPI custody with app-data records subordinate to a registry epoch.
use fs2::FileExt;
use getrandom::fill;
use ocentra_family_identity_core::trust_bootstrap::AwaitingPlatformKeySealingRequest;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

#[cfg(windows)]
#[path = "windows_device_trust_custody_active_record.rs"]
mod active_record;
#[cfg(windows)]
#[path = "windows_device_trust_custody_active_record_scan.rs"]
mod active_record_scan;
#[path = "windows_device_trust_custody_commitment.rs"]
mod commitment;
#[path = "windows_device_trust_custody_path.rs"]
mod path;
#[path = "windows_device_trust_custody_platform.rs"]
mod platform;
#[path = "windows_device_trust_custody_record.rs"]
mod record;
#[path = "windows_device_trust_custody_snapshot.rs"]
mod snapshot;
#[path = "windows_device_trust_custody_transaction.rs"]
mod transaction;
use record::{
    binding as custody_binding, hex, install_generation as load_or_create_install_generation,
    install_generation_fence, remove, write, Record,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Invalid,
    Platform,
    Missing,
    Mismatch,
    Io,
    Unseal,
}
pub struct WindowsDeviceTrustCustody {
    root: PathBuf,
    install_generation: String,
    binding_locks: Mutex<HashMap<String, Arc<Mutex<()>>>>,
}

impl WindowsDeviceTrustCustody {
    pub fn open(root: impl AsRef<Path>) -> Result<Self, Error> {
        if !cfg!(windows) {
            return Err(Error::Platform);
        }
        #[cfg(windows)]
        path::validate_custody_root_and_ancestors(root.as_ref())?;
        let _generation_fence = install_generation_fence(root.as_ref())?;
        let root_was_absent = !root.as_ref().exists();
        fs::create_dir_all(root.as_ref()).map_err(|_error| Error::Io)?;
        #[cfg(windows)]
        path::validate_custody_root_and_ancestors(root.as_ref())?;
        let root = root.as_ref().canonicalize().map_err(|_error| Error::Io)?;
        Ok(Self {
            install_generation: load_or_create_install_generation(&root, root_was_absent)?,
            root,
            binding_locks: Mutex::new(HashMap::new()),
        })
    }
    pub fn seal_persist_activate(
        &self,
        request: AwaitingPlatformKeySealingRequest,
        material: &[u8],
    ) -> Result<(), Error> {
        platform::require_authenticated_parent_authority()?;
        path::validate_seal_material(material)?;
        let _generation_fence = install_generation_fence(&self.root)?;
        let binding = custody_binding([
            &request.family_id,
            &request.parent_account_id,
            &request.device_ref,
            &self.install_generation,
        ])?;
        let binding_lock = self.binding_lock(&binding);
        let _binding_guard = binding_lock
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _binding_fence = self.binding_fence(&binding)?;
        let mut epoch = [0; 32];
        fill(&mut epoch).map_err(|_error| Error::Platform)?;
        let record = Record {
            family: request.family_id,
            account: request.parent_account_id,
            device: request.device_ref,
            epoch_hash: hex(Sha256::digest(epoch)),
            ciphertext: platform::protect(material, &binding)?,
        };
        let record_path = self.path(&binding);
        let previous = snapshot::preserve_active(&binding, &record_path)?;
        write(&record_path, &record)?;
        transaction::finish(
            commitment::write(&binding, &record_path),
            &binding,
            &record_path,
            &previous,
        )?;
        transaction::finish(
            platform::activate(&binding, &epoch),
            &binding,
            &record_path,
            &previous,
        )?;
        transaction::finish(
            verify_activated_binding(&epoch, platform::current(&binding)),
            &binding,
            &record_path,
            &previous,
        )?;
        #[cfg(windows)]
        transaction::finish(
            platform::mark_install_generation_sealed(
                &self.root,
                &self.install_generation,
                &binding,
            ),
            &binding,
            &record_path,
            &previous,
        )?;
        Ok(())
    }
    pub fn unseal_current(&self, family: &str, account: &str, device: &str) -> Result<(), Error> {
        platform::require_authenticated_parent_authority()?;
        let b = custody_binding([family, account, device, &self.install_generation])?;
        let binding_lock = self.binding_lock(&b);
        let _binding_guard = binding_lock
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _binding_fence = self.binding_fence(&b)?;
        let encoded = fs::read(self.path(&b)).map_err(|_error| Error::Missing)?;
        let r: Record = serde_json::from_slice(&encoded).map_err(|_error| Error::Io)?;
        if r.family != family
            || r.account != account
            || r.device != device
            || r.epoch_hash != hex(Sha256::digest(platform::current(&b)?))
        {
            return Err(Error::Mismatch);
        }
        commitment::verify(&b, &encoded)?;
        platform::unprotect(&r.ciphertext, &b).map(|_plaintext| ())
    }
    pub fn revoke_or_reset(&self, family: &str, account: &str, device: &str) -> Result<(), Error> {
        // Local callers cannot revoke parent trust without the owning
        // authenticated authority. Until that authority is available, keep
        // the operation manual-required and preserve the sealed material.
        platform::require_authenticated_parent_authority()?;
        let b = custody_binding([family, account, device, &self.install_generation])?;
        let binding_lock = self.binding_lock(&b);
        let _binding_guard = binding_lock
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _binding_fence = self.binding_fence(&b)?;
        match platform::remove(&b) {
            Ok(()) | Err(Error::Missing) => {}
            Err(error) => return Err(error),
        }
        remove(&self.path(&b))
    }
    fn path(&self, b: &[u8]) -> PathBuf {
        self.root.join(format!("{}.sealed", hex(Sha256::digest(b))))
    }

    fn binding_fence(&self, binding: &[u8]) -> Result<fs::File, Error> {
        let fence = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(
                self.root
                    .join(format!("{}.lock", hex(Sha256::digest(binding)))),
            )
            .map_err(|_error| Error::Io)?;
        fence.lock_exclusive().map_err(|_error| Error::Io)?;
        Ok(fence)
    }

    fn binding_lock(&self, binding: &[u8]) -> Arc<Mutex<()>> {
        let key = hex(Sha256::digest(binding));
        let mut locks = self
            .binding_locks
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        Arc::clone(locks.entry(key).or_insert_with(|| Arc::new(Mutex::new(()))))
    }
}

fn verify_activated_binding(
    epoch: &[u8],
    verification: Result<Vec<u8>, Error>,
) -> Result<(), Error> {
    match verification {
        Ok(current) if current == epoch => Ok(()),
        Ok(_) => Err(Error::Mismatch),
        Err(error) => Err(error),
    }
}

#[cfg(all(test, windows))]
mod tests {
    use super::{
        custody_binding, hex, install_generation_fence, platform, snapshot, transaction,
        verify_activated_binding, write, Error, Record, WindowsDeviceTrustCustody,
    };
    use sha2::{Digest, Sha256};
    use std::{fs, process::Command};

    #[test]
    fn verification_read_error_removes_the_persisted_record() -> Result<(), String> {
        let root = std::env::temp_dir().join(format!(
            "ocentra-wp02-post-activation-cleanup-{}",
            std::process::id()
        ));
        let _cleanup = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).map_err(|error| format!("create root: {error}"))?;
        let record_path = root.join("record.sealed");
        fs::write(&record_path, "sealed").map_err(|error| format!("write record: {error}"))?;

        assert_eq!(
            transaction::finish(
                verify_activated_binding(b"epoch", Err(Error::Platform)),
                b"post-activation-cleanup",
                &record_path,
                &None,
            ),
            Err(Error::Platform)
        );
        assert!(
            !record_path.exists(),
            "a verification read failure must remove the persisted record"
        );

        let _cleanup = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn generation_status_lock_keeps_a_concurrent_open_on_the_sealed_generation(
    ) -> Result<(), String> {
        let root = std::env::temp_dir().join(format!(
            "ocentra-wp02-generation-status-lock-{}",
            std::process::id()
        ));
        let _cleanup = fs::remove_dir_all(&root);
        let custody = WindowsDeviceTrustCustody::open(&root)
            .map_err(|error| format!("open original custody: {error:?}"))?;
        let generation = custody.install_generation.clone();
        let binding = custody_binding(["family", "account", "device", &generation])
            .map_err(|error| format!("derive binding: {error:?}"))?;
        let epoch = [6_u8; 32];
        write(
            &custody.path(&binding),
            &Record {
                family: "family".to_owned(),
                account: "account".to_owned(),
                device: "device".to_owned(),
                epoch_hash: hex(Sha256::digest(epoch)),
                ciphertext: platform::protect(b"sealed-material", &binding)
                    .map_err(|error| format!("protect material: {error:?}"))?,
            },
        )
        .map_err(|error| format!("write active record: {error:?}"))?;
        platform::activate(&binding, &epoch)
            .map_err(|error| format!("activate epoch: {error:?}"))?;
        assert_eq!(
            platform::current(&binding).map_err(|error| format!("read epoch: {error:?}"))?,
            epoch
        );
        let fence = install_generation_fence(&custody.root)
            .map_err(|error| format!("lock install generation: {error:?}"))?;
        let concurrent_root = custody.root.clone();
        let (entered_sender, entered_receiver) = std::sync::mpsc::channel();
        let opener = std::thread::spawn(move || {
            entered_sender.send(()).map_err(|error| error.to_string())?;
            WindowsDeviceTrustCustody::open(concurrent_root)
                .map_err(|error| format!("open: {error:?}"))
        });
        entered_receiver
            .recv()
            .map_err(|error| format!("wait for opener: {error}"))?;
        platform::mark_install_generation_sealed(&custody.root, &generation, &binding)
            .map_err(|error| format!("mark sealed generation: {error:?}"))?;
        drop(fence);
        let opened = opener
            .join()
            .map_err(|_panic| "concurrent opener panicked".to_owned())??;
        assert_eq!(opened.install_generation, generation);

        let _cleanup = platform::remove(&binding);
        let _cleanup = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn failed_post_activation_status_restores_the_prior_active_record() -> Result<(), String> {
        let root = std::env::temp_dir().join(format!(
            "ocentra-wp02-reseal-restore-{}",
            std::process::id()
        ));
        let _cleanup = fs::remove_dir_all(&root);
        let custody = WindowsDeviceTrustCustody::open(&root)
            .map_err(|error| format!("open custody: {error:?}"))?;
        let binding = custody_binding(["family", "account", "device", &custody.install_generation])
            .map_err(|error| format!("derive binding: {error:?}"))?;
        let record_path = custody.path(&binding);
        let old_epoch = [9_u8; 32];
        let old_record = Record {
            family: "family".to_owned(),
            account: "account".to_owned(),
            device: "device".to_owned(),
            epoch_hash: hex(Sha256::digest(old_epoch)),
            ciphertext: platform::protect(b"old-material", &binding)
                .map_err(|error| format!("protect old material: {error:?}"))?,
        };
        write(&record_path, &old_record).map_err(|error| format!("write old record: {error:?}"))?;
        platform::activate(&binding, &old_epoch)
            .map_err(|error| format!("activate old epoch: {error:?}"))?;
        let previous = snapshot::preserve_active(&binding, &record_path)
            .map_err(|error| format!("preserve old record: {error:?}"))?;

        let new_epoch = [10_u8; 32];
        write(
            &record_path,
            &Record {
                epoch_hash: hex(Sha256::digest(new_epoch)),
                ciphertext: platform::protect(b"new-material", &binding)
                    .map_err(|error| format!("protect new material: {error:?}"))?,
                ..old_record.clone()
            },
        )
        .map_err(|error| format!("write new record: {error:?}"))?;
        platform::activate(&binding, &new_epoch)
            .map_err(|error| format!("activate new epoch: {error:?}"))?;
        assert_eq!(
            transaction::finish(Err(Error::Platform), &binding, &record_path, &previous),
            Err(Error::Platform)
        );
        assert!(
            platform::current(&binding)
                .map_err(|error| format!("read restored epoch: {error:?}"))?
                == old_epoch,
            "the prior registry epoch must be restored"
        );
        let restored: Record = serde_json::from_slice(
            &fs::read(&record_path).map_err(|error| format!("read restored record: {error}"))?,
        )
        .map_err(|error| format!("parse restored record: {error}"))?;
        assert_eq!(restored.epoch_hash, old_record.epoch_hash);

        let _cleanup = platform::remove(&binding);
        let _cleanup = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn restored_valid_record_cannot_reuse_the_epoch_after_root_identity_changes(
    ) -> Result<(), String> {
        let root = std::env::temp_dir().join(format!(
            "ocentra-wp02-valid-restored-record-{}",
            std::process::id()
        ));
        let _cleanup = fs::remove_dir_all(&root);
        let custody = WindowsDeviceTrustCustody::open(&root)
            .map_err(|error| format!("open original custody: {error:?}"))?;
        let binding = custody_binding(["family", "account", "device", &custody.install_generation])
            .map_err(|error| format!("derive original binding: {error:?}"))?;
        let epoch = [7_u8; 32];
        let record_path = custody.path(&binding);
        write(
            &record_path,
            &Record {
                family: "family".to_owned(),
                account: "account".to_owned(),
                device: "device".to_owned(),
                epoch_hash: hex(Sha256::digest(epoch)),
                ciphertext: platform::protect(b"sealed-material", &binding)
                    .map_err(|error| format!("protect material: {error:?}"))?,
            },
        )
        .map_err(|error| format!("write valid record: {error:?}"))?;
        platform::activate(&binding, &epoch)
            .map_err(|error| format!("activate original epoch: {error:?}"))?;
        let restored_record =
            fs::read(&record_path).map_err(|error| format!("backup record: {error}"))?;

        fs::remove_dir_all(&root).map_err(|error| format!("remove custody root: {error}"))?;
        fs::create_dir_all(&root).map_err(|error| format!("recreate custody root: {error}"))?;
        fs::write(&record_path, restored_record)
            .map_err(|error| format!("restore valid record: {error}"))?;
        let restored = WindowsDeviceTrustCustody::open(&root)
            .map_err(|error| format!("open restored custody: {error:?}"))?;

        assert_eq!(
            restored.unseal_current("family", "account", "device"),
            Err(Error::Platform)
        );

        let _cleanup = platform::remove(&binding);
        let _cleanup = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn lost_sealed_content_rotates_generation_before_an_old_record_can_return() -> Result<(), String>
    {
        let root = std::env::temp_dir().join(format!(
            "ocentra-wp02-lost-sealed-content-{}",
            std::process::id()
        ));
        let _cleanup = fs::remove_dir_all(&root);
        let custody = WindowsDeviceTrustCustody::open(&root)
            .map_err(|error| format!("open original custody: {error:?}"))?;
        let old_generation = custody.install_generation.clone();
        let binding = custody_binding(["family", "account", "device", &old_generation])
            .map_err(|error| format!("derive original binding: {error:?}"))?;
        let epoch = [8_u8; 32];
        let record_path = custody.path(&binding);
        write(
            &record_path,
            &Record {
                family: "family".to_owned(),
                account: "account".to_owned(),
                device: "device".to_owned(),
                epoch_hash: hex(Sha256::digest(epoch)),
                ciphertext: platform::protect(b"sealed-material", &binding)
                    .map_err(|error| format!("protect material: {error:?}"))?,
            },
        )
        .map_err(|error| format!("write valid record: {error:?}"))?;
        platform::activate(&binding, &epoch)
            .map_err(|error| format!("activate original epoch: {error:?}"))?;
        platform::mark_install_generation_sealed(&custody.root, &old_generation, &binding)
            .map_err(|error| format!("mark sealed generation: {error:?}"))?;
        let restored_record =
            fs::read(&record_path).map_err(|error| format!("backup record: {error}"))?;

        fs::remove_file(&record_path).map_err(|error| format!("lose sealed record: {error}"))?;
        let after_loss = WindowsDeviceTrustCustody::open(&root)
            .map_err(|error| format!("open after content loss: {error:?}"))?;
        assert_ne!(old_generation, after_loss.install_generation);
        fs::write(&record_path, restored_record)
            .map_err(|error| format!("restore old record: {error}"))?;
        assert_eq!(
            after_loss.unseal_current("family", "account", "device"),
            Err(Error::Platform)
        );

        let _cleanup = platform::remove(&binding);
        let _cleanup = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn custody_rejects_root_and_ancestor_junctions() -> Result<(), String> {
        let base = std::env::temp_dir().join(format!(
            "ocentra-wp02-custody-junction-{}",
            std::process::id()
        ));
        let _cleanup = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).map_err(|error| format!("create base: {error}"))?;
        let target = base.join("target");
        fs::create_dir_all(&target).map_err(|error| format!("create target: {error}"))?;
        let root_junction = base.join("root-junction");
        let status = Command::new("cmd")
            .args(["/C", "mklink", "/J"])
            .arg(&root_junction)
            .arg(&target)
            .status()
            .map_err(|error| format!("create root junction: {error}"))?;
        if !status.success() {
            return Err("create root junction command failed".to_owned());
        }
        assert!(matches!(
            WindowsDeviceTrustCustody::open(&root_junction),
            Err(Error::Invalid)
        ));
        fs::remove_dir(&root_junction).map_err(|error| format!("remove root junction: {error}"))?;

        let ancestor_target = base.join("ancestor-target");
        fs::create_dir_all(&ancestor_target)
            .map_err(|error| format!("create ancestor target: {error}"))?;
        let ancestor_junction = base.join("ancestor-junction");
        let status = Command::new("cmd")
            .args(["/C", "mklink", "/J"])
            .arg(&ancestor_junction)
            .arg(&ancestor_target)
            .status()
            .map_err(|error| format!("create ancestor junction: {error}"))?;
        if !status.success() {
            return Err("create ancestor junction command failed".to_owned());
        }
        assert!(matches!(
            WindowsDeviceTrustCustody::open(ancestor_junction.join("custody")),
            Err(Error::Invalid)
        ));
        fs::remove_dir(&ancestor_junction)
            .map_err(|error| format!("remove ancestor junction: {error}"))?;

        let _cleanup = fs::remove_dir_all(base);
        Ok(())
    }
}
