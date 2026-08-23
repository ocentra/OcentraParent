#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use serde::{Deserialize, Serialize};

// The identifier is visible only through this crate's private record module.
// A request caller cannot construct or deserialize a current session record.
family_identity_text_id!(SessionId, "family_identity.session_id");
