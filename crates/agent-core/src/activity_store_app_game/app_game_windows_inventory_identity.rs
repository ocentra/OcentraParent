use std::collections::HashSet;

use super::WindowsInstalledAppInventoryRecord;

pub(super) fn strong_identity_seen(
    record: &WindowsInstalledAppInventoryRecord,
    strong_identities: &mut HashSet<(u8, String)>,
) -> bool {
    let keys = strong_identity_keys(record);
    if keys.is_empty() {
        return false;
    }
    if keys.iter().any(|key| strong_identities.contains(key)) {
        return true;
    }
    strong_identities.extend(keys);
    false
}

fn strong_identity_keys(record: &WindowsInstalledAppInventoryRecord) -> Vec<(u8, String)> {
    let mut keys = Vec::new();
    push_identity_key(&mut keys, 1, &record.identity_id);
    push_identity_key(&mut keys, 2, &record.package_id);
    push_identity_key(&mut keys, 3, &record.bundle_id);
    push_identity_key(&mut keys, 4, &record.app_user_model_id);
    push_identity_key(&mut keys, 5, &record.executable_path_ref);
    push_identity_key(&mut keys, 6, &record.launcher_app_id);
    push_identity_key(&mut keys, 7, &record.launcher_manifest_id);
    push_identity_key(&mut keys, 8, &record.store_id);
    push_identity_key(&mut keys, 9, &record.catalog_ref);
    push_identity_key(&mut keys, 10, &record.desktop_entry_id);
    keys
}

fn push_identity_key(keys: &mut Vec<(u8, String)>, rank: u8, value: &Option<String>) {
    if let Some(value) = value {
        keys.push((rank, value.clone()));
    }
}
