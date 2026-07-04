use super::*;

pub(super) fn bundle_id(value: &str) -> ChildIosEntitlementBundleId {
    crate::schema_option_or_unreachable(
        ChildIosEntitlementBundleId::parse(value),
        "child iOS entitlement bundle id",
    )
}

pub(super) fn class_name(value: &str) -> ChildIosEntitlementClassName {
    crate::schema_option_or_unreachable(
        ChildIosEntitlementClassName::parse(value),
        "child iOS entitlement class name",
    )
}

pub(super) fn requirement(value: &str) -> ChildIosEntitlementRequirement {
    crate::schema_option_or_unreachable(
        ChildIosEntitlementRequirement::parse(value),
        "child iOS entitlement requirement",
    )
}

pub(super) fn boundary(value: &str) -> ChildIosEntitlementBoundary {
    crate::schema_option_or_unreachable(
        ChildIosEntitlementBoundary::parse(value),
        "child iOS entitlement boundary",
    )
}

pub(super) fn timestamp(value: &str) -> ChildIosEntitlementTimestamp {
    crate::schema_option_or_unreachable(
        ChildIosEntitlementTimestamp::parse(value),
        "child iOS entitlement timestamp",
    )
}
