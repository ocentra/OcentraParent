use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;

use crate::ActivityStore;

#[test]
fn activity_store_returns_no_enforcement_audit_fields_when_empty() {
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);

    let fields = store
        .latest_enforcement_audit_fields()
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(fields, None);
}
