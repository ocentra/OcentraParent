use super::*;

pub(super) fn begin(
    payload: &Value,
    read_model: &ParentPolicyPreviewReadModelSnapshot,
    parent_access_state: &ParentPortalParentAccessState,
    lan_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> Result<StagedParentResolution, String> {
    let validated =
        super::validation::validate(payload, read_model, parent_access_state, lan_read_model)?;
    let handle = super::lifecycle::opaque_handle()?;
    let (request, stored) = super::request::build(&validated, &handle);
    super::lifecycle::store_new(&handle, stored)?;
    Ok(StagedParentResolution {
        handle,
        preview_id: validated.preview_id,
        parent_actor_id: validated.parent_actor_id,
        request,
    })
}
