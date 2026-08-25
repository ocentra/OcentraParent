use serde::Deserialize;

use super::{AiChildProfileId, AiDeviceId, AiFamilyId, AiSubjectIdentity};

impl AiSubjectIdentity {
    pub fn new(
        family_id: AiFamilyId,
        child_profile_id: Option<AiChildProfileId>,
        device_id: Option<AiDeviceId>,
    ) -> Result<Self, &'static str> {
        Ok(Self {
            family_id,
            child_profile_id,
            device_id,
        })
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn child_profile_id(&self) -> Option<&AiChildProfileId> {
        self.child_profile_id.as_ref()
    }

    pub fn device_id(&self) -> Option<&AiDeviceId> {
        self.device_id.as_ref()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiSubjectIdentityFields {
    family_id: AiFamilyId,
    child_profile_id: Option<AiChildProfileId>,
    device_id: Option<AiDeviceId>,
}

impl<'de> Deserialize<'de> for AiSubjectIdentity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = AiSubjectIdentityFields::deserialize(deserializer)?;
        Self::new(fields.family_id, fields.child_profile_id, fields.device_id)
            .map_err(serde::de::Error::custom)
    }
}
