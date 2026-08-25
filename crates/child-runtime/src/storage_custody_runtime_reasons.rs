use ocentra_storage_custody_core::storage_custody::StorageCustodyEffectKind;

pub(crate) fn manual_required_reason(effect: StorageCustodyEffectKind) -> &'static str {
    match effect {
        StorageCustodyEffectKind::ParentOwnedSync => {
            "parent-owned provider connector/upload executor is not composed"
        }
        StorageCustodyEffectKind::Export | StorageCustodyEffectKind::Backup => {
            "parent-owned encrypted export executor is not composed"
        }
        StorageCustodyEffectKind::ParentOwnedDelete => {
            "parent-owned provider delete executor is not composed"
        }
        StorageCustodyEffectKind::Import => {
            "restore/import executor is unavailable; preview remains non-mutating"
        }
        StorageCustodyEffectKind::ReportQuery => {
            "report/query producer and parent read model are owned by another service"
        }
        StorageCustodyEffectKind::SettingsApply => {
            "parent storage settings apply owner is not composed"
        }
        StorageCustodyEffectKind::LocalDelete => "local delete executor was not available",
    }
}
