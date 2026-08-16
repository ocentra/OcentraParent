use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEventKind, LanDiscoveryEventRow,
};
use std::primitive::str as TestStr;
use std::string::String as TestString;

type TestText = TestString;

pub(super) fn discovery_row<'a>(
    rows: &'a [LanDiscoveryEventRow],
    event_kind: &LanDiscoveryEventKind,
    affected_device_id: Option<&TestText>,
    evidence_id: Option<&TestText>,
    context: &'static TestStr,
) -> &'a LanDiscoveryEventRow {
    crate::test_invariants::require_some(
        rows.iter().find(|row| {
            row.event_kind == *event_kind
                && row.affected_device_id.as_ref() == affected_device_id
                && row.evidence_id.as_ref() == evidence_id
        }),
        context,
    )
}

pub(super) fn assert_row_contract(
    row: &LanDiscoveryEventRow,
    scan_id: &TestText,
    affected_device_id: Option<&TestText>,
    evidence_id: Option<&TestText>,
    event_id: &TestText,
    occurred_at: &'static TestStr,
) {
    assert_eq!(row.scan_session_id.as_ref(), Some(scan_id));
    assert_eq!(row.affected_device_id.as_ref(), affected_device_id);
    assert_eq!(row.evidence_id.as_ref(), evidence_id);
    assert_eq!(row.event_id.as_str(), event_id.as_str());
    assert_eq!(row.occurred_at.as_str(), occurred_at);
}

pub(super) fn assert_first_row_has_no_previous_event(rows: &[LanDiscoveryEventRow]) {
    assert_eq!(
        rows.first().and_then(|row| row.previous_event_id.as_ref()),
        None
    );
}

pub(super) fn assert_previous_event_chain(rows: &[LanDiscoveryEventRow]) {
    for row_pair in rows.windows(2) {
        assert_eq!(
            row_pair[1].previous_event_id.as_deref(),
            Some(row_pair[0].event_id.as_str())
        );
    }
}
