# Authority Tier Proof

The contract supports these authority tiers:

- observe-only
- user-approved-helper
- accessibility-assisted
- managed-profile
- device-owner
- mdm-enrolled
- supervised-device
- system-extension
- root-or-admin-service
- kiosk-or-single-app
- manual-required
- not-claimed

Every row names:

- platform;
- action;
- authority tier;
- setup state;
- proof state;
- enforcement capability state;
- parent-visible state and limitation;
- whether an adapter can execute;
- supported enforcement modes;
- attached proof references;
- proof needed to claim.

Linux hard-block rows also require mechanism, distro, and session fields before
they can claim support.
