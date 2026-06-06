# SOCIAL-17 Security Negative Proof

SOCIAL-17 rejects iOS Screen Time/ManagedSettings overclaims:

- Apple entitlement approval;
- raw app identity;
- native social route proof;
- per-video or per-reel blocking;
- message content;
- account identity;
- screen content capture;
- DeviceActivity runtime behavior;
- ManagedSettings runtime behavior;
- platform connector access;
- UI delivery;
- enforcement.

FamilyControls token rows remain opaque-token and manual-device-proof oriented
until real Apple entitlement, family authorization, token selection, signing,
and physical device proof exist. ManagedSettings rows remain shield-capability
mapping only and do not prove that shields were applied.

The host proof also runs negative matrix mutations for runtime and overclaim
fields. It records missing Apple/iOS tooling on this Windows host as
`host-tooling-unavailable`; that evidence cannot be promoted into entitlement,
token, route, content, UI, connector, or enforcement proof.
