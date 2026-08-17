use super::{StorageCustodyEffect, StorageCustodyEffectKind};

impl StorageCustodyEffect {
    pub fn kind(&self) -> StorageCustodyEffectKind {
        match self {
            Self::DeleteLocal { .. } => StorageCustodyEffectKind::LocalDelete,
            Self::ParentOwnedSync => StorageCustodyEffectKind::ParentOwnedSync,
            Self::Export => StorageCustodyEffectKind::Export,
            Self::Import => StorageCustodyEffectKind::Import,
            Self::Backup => StorageCustodyEffectKind::Backup,
            Self::ParentOwnedDelete => StorageCustodyEffectKind::ParentOwnedDelete,
            Self::ReportQuery => StorageCustodyEffectKind::ReportQuery,
            Self::SettingsApply => StorageCustodyEffectKind::SettingsApply,
        }
    }

    pub fn reference(&self) -> String {
        format!("{:?}", self.kind())
    }
}
