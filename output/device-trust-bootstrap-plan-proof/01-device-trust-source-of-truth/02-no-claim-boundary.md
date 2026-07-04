## No-claim boundary

- this proof root covers WP01 source-of-truth ownership only
- it does not claim local key sealing, parent step-up auth, phone QR approval, entitlement-device license, recovery/reset, child tamper/uninstall, or route-gate completion
- it does not claim device trust ready, whole-plan ready, or PR ready
- it does not turn login, package install, LAN pairing, or license presence into trust proof
- it does not claim any TS implementation owns provisioning logic; the Rust crate owns the readiness boundary here
- it does not use inline tests, `.gitkeep` trees, or empty folders as evidence
