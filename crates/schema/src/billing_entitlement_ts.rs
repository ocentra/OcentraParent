pub const BILLING_ENTITLEMENT_TYPESCRIPT: &str = concat!(
    include_str!("billing_entitlement.template.txt"),
    include_str!("billing_entitlement.snapshot-and-device-limit.template.txt"),
    include_str!("billing_entitlement.proof-and-exports.template.txt"),
);
