use super::{AccountIdentityAuthorityService, SqliteAccountIdentityAuthorityRepository};

/// Account-owned monotonic CAS. The caller can only advance a row when it
/// presents the durable generation/session pair it just read; every advance
/// rotates the current session identity and increments both generations. No
/// selector or request header can mint a row.
impl SqliteAccountIdentityAuthorityRepository {}

impl AccountIdentityAuthorityService {}
