use crate::billing_child_entitlement::{
    BillingChildEntitlementConsumptionDecision, BillingChildEntitlementSnapshot,
};

mod construct;
mod dispatch;
mod freshness;

pub(crate) fn decide_child_entitlement_snapshot(
    snapshot: BillingChildEntitlementSnapshot,
) -> BillingChildEntitlementConsumptionDecision {
    dispatch::decide_child_entitlement_snapshot(snapshot)
}
