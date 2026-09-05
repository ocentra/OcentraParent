#![cfg(test)]

#[test]
fn passive_cycle_cursor_rotates_all_six_listeners_fairly() {
    let mut cursor = super::cycle_cursor::PassiveDiscoveryCycleCursor::new(0, 6);
    for expected in 0..6 {
        assert_eq!(cursor.take_next(), Some(expected));
    }
    assert_eq!(cursor.take_next(), Some(0));
    let mut offset = super::cycle_cursor::PassiveDiscoveryCycleCursor::new(4, 6);
    let sequence = (0..6).map(|_| offset.take_next()).collect::<Vec<_>>();
    assert_eq!(
        sequence,
        vec![Some(4), Some(5), Some(0), Some(1), Some(2), Some(3)]
    );
    let mut partial = super::cycle_cursor::PassiveDiscoveryCycleCursor::new(2, 6);
    assert_eq!(partial.take_next(), Some(2));
    assert_eq!(partial.resume_index(), 3);
    let mut empty = super::cycle_cursor::PassiveDiscoveryCycleCursor::new(0, 0);
    assert_eq!(empty.take_next(), None);
    assert_eq!(empty.resume_index(), 0);
}

#[test]
fn passive_cycle_cursor_honors_shared_budget_and_cancellation() {
    assert!(
        super::cycle_cursor::PassiveDiscoveryCycleCursor::should_continue(
            true,
            0,
            6,
            std::time::Duration::from_millis(1),
        )
    );
    assert!(
        !super::cycle_cursor::PassiveDiscoveryCycleCursor::should_continue(
            true,
            6,
            6,
            std::time::Duration::from_millis(1),
        )
    );
    assert!(
        !super::cycle_cursor::PassiveDiscoveryCycleCursor::should_continue(
            true,
            0,
            6,
            std::time::Duration::ZERO,
        )
    );
    assert!(
        !super::cycle_cursor::PassiveDiscoveryCycleCursor::should_continue(
            false,
            0,
            6,
            std::time::Duration::from_millis(1),
        )
    );
}
