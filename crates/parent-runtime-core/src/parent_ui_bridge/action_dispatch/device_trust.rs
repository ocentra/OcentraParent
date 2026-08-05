use serde::Deserialize;

use super::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DeviceTrustSealPayload {
    ceremony_ref: String,
}

pub(super) fn dispatch_parent_ui_action_device_trust(
    action: &ParentUiAction,
    device_trust: Option<&ParentDeviceTrustCommandFacade>,
    state: &mut ActionDispatchState,
) {
    if !matches!(
        action.action,
        ParentUiActionKind::DeviceTrustSealStagedCeremonyRequested
    ) {
        return;
    }

    let payload = serde_json::from_value::<DeviceTrustSealPayload>(action.payload.clone())
        .map_err(|_error| "device-trust seal payload is invalid")
        .and_then(|payload| {
            if payload.ceremony_ref.trim().is_empty() {
                Err("device-trust ceremony reference is required")
            } else {
                Ok(payload)
            }
        });
    let result = payload.and_then(|payload| {
        device_trust
            .ok_or("device-trust native command facade is unavailable")
            .and_then(|facade| {
                facade
                    .seal_staged_parent_device_trust(&payload.ceremony_ref)
                    .map_err(|_error| "device-trust staged ceremony was rejected")
            })
    });
    if let Err(error) = result {
        state.reject(error);
    }
}
