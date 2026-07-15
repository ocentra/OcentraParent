use ocentra_schema::encryption_key_custody as contracts;

use super::{EncryptionKeyCustodyDerivationError, PlatformKeyCustodyInput};

pub(super) fn derive_platform_key_custody_row(
    input: PlatformKeyCustodyInput,
) -> Result<contracts::PlatformKeyCustodyRow, EncryptionKeyCustodyDerivationError> {
    match input.surface {
        contracts::PlatformKeyCustodySurface::Linux => {
            if !input.manual_required
                || input.decrypt_authority != contracts::PlatformDecryptAuthority::ManualRequired
            {
                return Err(EncryptionKeyCustodyDerivationError::LinuxMustStayManualRequired);
            }
        }
        contracts::PlatformKeyCustodySurface::Android
        | contracts::PlatformKeyCustodySurface::IOS
        | contracts::PlatformKeyCustodySurface::ParentMobile
        | contracts::PlatformKeyCustodySurface::ChildMobile => {
            if !input.manual_required || !input.device_proof_required {
                return Err(
                    EncryptionKeyCustodyDerivationError::MobileProofGapMustStayManualRequired(
                        input.surface,
                    ),
                );
            }
        }
        contracts::PlatformKeyCustodySurface::WebPortal => {
            if input.decrypt_authority != contracts::PlatformDecryptAuthority::NotDecryptRoot {
                return Err(EncryptionKeyCustodyDerivationError::HostedPortalCannotDecrypt);
            }
        }
        _ => {}
    }

    Ok(contracts::PlatformKeyCustodyRow {
        surface: input.surface,
        key_store: input.key_store,
        decrypt_authority: input.decrypt_authority,
        manual_required: input.manual_required,
        device_proof_required: input.device_proof_required,
        wrong_household_fails_closed: true,
        wrong_device_fails_closed: true,
        revoked_key_fails_closed: true,
        notes: input.notes,
    })
}
