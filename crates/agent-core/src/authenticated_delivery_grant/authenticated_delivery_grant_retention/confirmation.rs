pub(super) fn is_current(
    authenticated_issued_at_nanos: Option<i64>,
    now: i64,
    highest: i64,
) -> bool {
    authenticated_issued_at_nanos.is_some_and(|issued_at| issued_at <= now && issued_at <= highest)
}

pub(super) fn provisional_observed_at(now: i64, highest: i64, observed_at: i64) -> i64 {
    if now < highest {
        now
    } else {
        observed_at
    }
}
