<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `Account Identity Family Plan Checklist Index`
> Kind: exact checklist router.
> Read when: a selected workpack references checklist rows.
> Stop rule: do not scan unrelated checklist rows.
> Proves: checklist routing only.
> Does not prove: implementation completion.
> Proof rule: a checkbox can be checked only after proof artifacts and focused command results exist.

<!-- /agent-capsule -->

# Account Identity Family Plan Checklist Index

> **Live-code audit (2026-07-17):** [Project Progress Matrix](../../PLAN_CODE_STATUS_MATRIX.md) records current implementation, blockers, dependencies, and next unblocker. Rows remain proof-gated; this audit does not check unsupported work.

## Fill rules

- Leave a checkbox unchecked until proof exists.
- Every checked row must cite one or more proof artifacts from `PROOF_INDEX.md`.
- Every proof item must list exact commands run, pass/fail/blocker, and no-claim boundaries.
- Do not mark provider/session/security/UI rows complete from docs-only work unless the workpack explicitly allows docs-only proof.
- Do not mark PR_READY until WP06 aggregates proof from all required earlier workpacks.

## WP01 Auth Provider Decision

- [x] Source-backed provider decision record written. Proof: `output/account-identity-family-plan-proof/01-auth-provider-decision/00-provider-decision-record.md`.
- [x] Cloudflare custody boundary accepted or blocker recorded. Proof: `output/account-identity-family-plan-proof/01-auth-provider-decision/02-provider-custody-boundary-proof.md`.
- [x] Firebase/Auth.js/other provider role accepted, rejected, or staged. Proof: `output/account-identity-family-plan-proof/01-auth-provider-decision/00-provider-decision-record.md`; `output/account-identity-family-plan-proof/01-auth-provider-decision/01-provider-rejected-options.md`.
- [x] Custom claims data-minimization rule documented. Proof: `output/account-identity-family-plan-proof/01-auth-provider-decision/03-custom-claims-data-minimization-proof.md`.
- [x] Identity provider cannot own household/product data. Proof: `output/account-identity-family-plan-proof/01-auth-provider-decision/01-provider-rejected-options.md`; `output/account-identity-family-plan-proof/01-auth-provider-decision/03-custom-claims-data-minimization-proof.md`.
- [x] Provider outage/degraded state documented. Proof: `output/account-identity-family-plan-proof/01-auth-provider-decision/04-provider-outage-degraded-proof.md`.
- [x] Migration/replaceability path documented. Proof: `output/account-identity-family-plan-proof/01-auth-provider-decision/05-migration-path-proof.md`.
- [x] Dev-mode bypass cannot reach production proof written. Proof: `output/account-identity-family-plan-proof/01-auth-provider-decision/01-provider-rejected-options.md`.
- [x] Required proof artifacts written. Proof: `output/account-identity-family-plan-proof/01-auth-provider-decision/00-provider-decision-record.md`; `output/account-identity-family-plan-proof/01-auth-provider-decision/01-provider-rejected-options.md`; `output/account-identity-family-plan-proof/01-auth-provider-decision/02-provider-custody-boundary-proof.md`; `output/account-identity-family-plan-proof/01-auth-provider-decision/03-custom-claims-data-minimization-proof.md`; `output/account-identity-family-plan-proof/01-auth-provider-decision/04-provider-outage-degraded-proof.md`; `output/account-identity-family-plan-proof/01-auth-provider-decision/05-migration-path-proof.md`; `output/account-identity-family-plan-proof/01-auth-provider-decision/16-validation-commands.log`.
- [x] Workpack completion section filled. Proof: `docs/plans/account-identity-family-plan/workpacks/01-auth-provider-decision.md`.

## WP08 Rust Schema And Account Authority

WP08 is complete only for its Rust-owned authority and generated-edge slice,
with a tracked hand-authored durable manifest under
`docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/`.
This does not assert a Cloudflare runtime, migration, deployment, account final
gate, or whole-plan completion.

Current production-handoff overlay (the checked historical packet below does
not satisfy these rows):

- [ ] WP02 resolves actor parent-controller and target child/profile/device
  separately and derives same-family, capability, lease, and step-up authority
  from owned state.
- [ ] Cloudflare WP06 owns authoritative Account D1
  write/update/revocation/CAS and a shipped Firebase/provider-to-sealed-
  authority caller; its current read adapter is not closure.
- [ ] Device Trust WP03 consumes live Account and Device Trust currentness;
  mapped contracts and prior proof are not runtime reachability.

- [x] Rust-owned canonical account/family schema and compatibility boundary exists, including the encoded TS-edge artifact `packages/schema-domain/src/generated-family-references.ts` generated from `crates/schema/src/family_references_ts.rs` and the matching contract drift test. Proof: `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/00-rust-schema-authority-proof.md`.
- [x] Account, household, membership, role, device, invite/recovery, and session authority paths preserve canonical schema ownership. Proof: `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/01-account-authority-parity-proof.md`.
- [x] Cross-household, revoked, stale, malformed, duplicate, and schema-incompatible authority cases reject or degrade safely. Proof: `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/02-account-authority-negative-proof.md`.
- [x] Redacted correlated authority proof covers account, household, device, invite, recovery, and session decisions without a worker-runtime claim. Proof: `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/03-redacted-authority-proof.md`.
- [x] Cloudflare WP06 storage handoff is recorded as a consumer of the canonical contract, not an Account WP08 implementation duty. Proof: `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/04-cloudflare-wp06-wp08-handoff.md`.
- [x] Cloudflare WP08 runner/proof follows Cloudflare WP06 and is not claimed as Account WP08 validation. Proof: `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/04-cloudflare-wp06-wp08-handoff.md`.
- [x] Focused Rust validation commands pass or precise blockers are recorded. Proof: `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/16-validation-commands.md`.
- [x] Required proof artifacts, no-claim boundary, and cross-plan handoff record exist. Proof: `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/00-rust-schema-authority-proof.md`; `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/04-cloudflare-wp06-wp08-handoff.md`; `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/05-no-claim-boundary.md`; `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/16-validation-commands.md`.
- [x] Workpack completion section is filled only after all prior WP08 rows are proven. Proof: `docs/plans/account-identity-family-plan/workpacks/08-rust-schema-workers-d1-runtime-migration.md`.

## WP02 Identity Household Role Model

Current production-closure overlay (the checked rows below retain historical
contract/proof evidence and do not satisfy these rows):

- [ ] A target-aware Account resolver keeps the actor parent-controller device
  distinct from the target child/profile/device for Pair, Register, Revoke,
  View, ChangePolicy, and Remote actions; callers cannot supply `same_family`,
  capability, controller lease, step-up, support, or lifecycle authority.
- [ ] ParentOwner, CoParent, and Observer `ViewChildStatus` is evaluated as a
  parent action over an independently resolved child/profile/device target.
- [ ] Cloudflare WP06 supplies the authoritative D1 writer/currentness/
  revocation/CAS owner and provider-to-sealed-authority production caller.
- [ ] Expected tests cover actor/target mismatch, cross-child/cross-household
  targets, caller-supplied trust rejection, correct parent `ViewChildStatus`,
  repository reload/concurrency, membership-state denial, audited support
  scope, and production-caller use of the sealed binding.

- [x] Account user model defined. Proof: `output/account-identity-family-plan-proof/02-identity-household-role-model/00-identity-entity-model-proof.md`.
- [x] Household model defined. Proof: `output/account-identity-family-plan-proof/02-identity-household-role-model/00-identity-entity-model-proof.md`.
- [x] Membership state machine defined. Proof: `output/account-identity-family-plan-proof/02-identity-household-role-model/02-membership-state-machine-proof.md`.
- [x] Role/action/resource matrix defined. Proof: `output/account-identity-family-plan-proof/02-identity-household-role-model/01-role-action-resource-matrix.md`.
- [x] Child profile and child device separated. Proof: `output/account-identity-family-plan-proof/02-identity-household-role-model/00-identity-entity-model-proof.md`.
- [x] Parent owner/co-parent/observer roles separated. Proof: `output/account-identity-family-plan-proof/02-identity-household-role-model/01-role-action-resource-matrix.md`; `output/account-identity-family-plan-proof/02-identity-household-role-model/04-observer-read-only-proof.md`.
- [x] Support/admin actor minimized and audited. Proof: `output/account-identity-family-plan-proof/02-identity-household-role-model/05-support-admin-boundary-proof.md`; `output/account-identity-family-plan-proof/02-identity-household-role-model/06-audit-event-proof.md`.
- [x] Pending/invited/revoked/disabled states represented. Proof: `output/account-identity-family-plan-proof/02-identity-household-role-model/02-membership-state-machine-proof.md`.
- [x] Cross-family negative proof exists. Proof: `output/account-identity-family-plan-proof/02-identity-household-role-model/03-cross-family-negative-proof.md`.
- [x] Observer read-only proof exists. Proof: `output/account-identity-family-plan-proof/02-identity-household-role-model/04-observer-read-only-proof.md`.
- [x] Audit-event requirement proof exists. Proof: `output/account-identity-family-plan-proof/02-identity-household-role-model/06-audit-event-proof.md`.
- [x] Required proof artifacts written. Proof: `output/account-identity-family-plan-proof/02-identity-household-role-model/00-identity-entity-model-proof.md`; `output/account-identity-family-plan-proof/02-identity-household-role-model/01-role-action-resource-matrix.md`; `output/account-identity-family-plan-proof/02-identity-household-role-model/02-membership-state-machine-proof.md`; `output/account-identity-family-plan-proof/02-identity-household-role-model/03-cross-family-negative-proof.md`; `output/account-identity-family-plan-proof/02-identity-household-role-model/04-observer-read-only-proof.md`; `output/account-identity-family-plan-proof/02-identity-household-role-model/05-support-admin-boundary-proof.md`; `output/account-identity-family-plan-proof/02-identity-household-role-model/06-audit-event-proof.md`; `output/account-identity-family-plan-proof/02-identity-household-role-model/16-validation-commands.log`.
- [x] Workpack completion section filled. Proof: `docs/plans/account-identity-family-plan/workpacks/02-identity-household-role-model.md`.

## WP03 Session Token Lifecycle

Current production-closure overlay (the checked rows below retain historical
contract/proof evidence and do not satisfy these rows):

- [ ] A durable session/refresh-family repository owns token digests, rotation
  generations, replay prevention, logout/global-revoke epochs, issued/expiry
  times, and real account request integration; lifecycle flags are not supplied
  by the caller.
- [ ] Expected tests cover atomic concurrent rotation, replay after restart,
  global revoke, clock skew, malformed/backdated state, exact audit emission,
  and CSRF/origin/fetch-metadata behavior on the real account browser route.

- [x] Credential type matrix defined. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/00-credential-type-matrix.md`.
- [x] Browser session lifecycle defined. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/01-session-lifecycle-proof.md`.
- [x] Refresh rotation defined. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/03-refresh-revocation-proof.md`.
- [x] Logout/global revoke behavior defined. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/01-session-lifecycle-proof.md`.
- [x] Expiry and clock-skew behavior defined. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/02-token-expiry-replay-proof.md`.
- [x] Reuse and stolen-token negative proof exists. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/02-token-expiry-replay-proof.md`.
- [x] Device credential separated from browser session. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/00-credential-type-matrix.md`.
- [x] Invite/recovery/controller-lease tokens separated from sessions. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/00-credential-type-matrix.md`.
- [x] Freshness requirement defined for sensitive actions. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/04-session-freshness-proof.md`.
- [x] State-changing request safety proof or blocker exists. Blocker: `output/account-identity-family-plan-proof/03-session-token-lifecycle/05-csrf-origin-proof.md`.
- [x] Redacted audit log proof exists. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/06-token-redaction-proof.md`.
- [x] Required proof artifacts written. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/00-credential-type-matrix.md`, `output/account-identity-family-plan-proof/03-session-token-lifecycle/01-session-lifecycle-proof.md`, `output/account-identity-family-plan-proof/03-session-token-lifecycle/02-token-expiry-replay-proof.md`, `output/account-identity-family-plan-proof/03-session-token-lifecycle/03-refresh-revocation-proof.md`, `output/account-identity-family-plan-proof/03-session-token-lifecycle/04-session-freshness-proof.md`, `output/account-identity-family-plan-proof/03-session-token-lifecycle/05-csrf-origin-proof.md`, `output/account-identity-family-plan-proof/03-session-token-lifecycle/06-token-redaction-proof.md`, `output/account-identity-family-plan-proof/03-session-token-lifecycle/16-validation-commands.log`.
- [x] Focused commands pass or blocker recorded. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/16-validation-commands.log`.
- [x] Workpack completion section filled. Proof: `docs/plans/account-identity-family-plan/workpacks/03-session-token-lifecycle.md`.

## WP04 Invites Recovery Lifecycle

Current production-closure overlay (the checked rows below retain historical
contract/proof evidence and do not satisfy these rows):

- [ ] A durable invite/recovery transition owner derives household/role/time,
  enforces atomic single use and monotonic terminal state, consumes opaque
  identity/owner/support authorization, and emits the typed custody handoff.
- [ ] Expected tests cover concurrent redemption, pre-issuance, expiry,
  revocation, replay, wrong household/role, rejected recovery non-advancement,
  enumeration/rate-limit behavior, restart, and audited support scope.

- [x] Invite state machine defined. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/00-invite-state-machine-proof.md`.
- [x] Co-parent/observer/child-device invite scopes separated. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/00-invite-state-machine-proof.md`.
- [x] Invite single-use proof exists. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/00-invite-state-machine-proof.md`.
- [x] Expired/revoked/reused invite negative proof exists. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/01-invite-negative-proof.md`.
- [x] Wrong-household/wrong-role invite negative proof exists. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/01-invite-negative-proof.md`.
- [x] Recovery state machine defined. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/02-recovery-state-machine-proof.md`.
- [x] Forgotten-login recovery modeled. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/02-recovery-state-machine-proof.md`.
- [x] Lost-parent-device and compromised-account recovery modeled. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/02-recovery-state-machine-proof.md`.
- [x] Child reinstall and household transfer modeled. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/02-recovery-state-machine-proof.md`.
- [x] Recovery rate-limit/enumeration-resistant proof exists. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/03-recovery-abuse-proof.md`.
- [x] Delete/export handoff to data custody documented. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/04-delete-export-handoff-proof.md`.
- [x] Support recovery audit proof exists. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/05-support-recovery-audit-proof.md`.
- [x] Workpack completion section filled. Proof: `docs/plans/account-identity-family-plan/workpacks/04-invites-recovery-lifecycle.md`.

## WP05 Device Ownership AuthZ

Current production-closure overlay (the checked rows below retain historical
contract/proof evidence and do not satisfy these rows):

- [ ] The production authorization composer derives current household,
  membership, device trust, session freshness, capability scope, controller
  lease, and step-up state from owned repositories/opaque receipts instead of a
  caller-provided flag bundle.
- [ ] Expected tests cover view-versus-control grant separation, lease identity
  and expiry, revoke/rebind races, required step-up consumption, audit emission,
  and typed remote/export/delete/billing consumers.

- [x] Actor/household/role/device/session/capability matrix defined. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/00-device-authority-matrix.md`.
- [x] Parent controller authority proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/00-device-authority-matrix.md`.
- [x] Parent observer read-only proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/04-remote-capability-proof.md`.
- [x] Child agent authority proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/00-device-authority-matrix.md`.
- [x] Pending/trusted/revoked/disabled/stale device states covered. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/00-device-authority-matrix.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/01-revoked-device-negative-proof.md`.
- [x] Wrong-household denial proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/02-wrong-household-negative-proof.md`.
- [x] Controller lease required/expired/revoked proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/03-controller-lease-proof.md`.
- [x] Remote view/control capability separation proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/04-remote-capability-proof.md`.
- [x] Export/delete owner-only proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/05-export-delete-owner-proof.md`.
- [x] Billing parent-owner proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/06-billing-owner-proof.md`.
- [x] Audit-event requirement proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/00-device-authority-matrix.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/06-billing-owner-proof.md`.
- [x] Required proof artifacts written. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/00-device-authority-matrix.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/01-revoked-device-negative-proof.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/02-wrong-household-negative-proof.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/03-controller-lease-proof.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/04-remote-capability-proof.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/05-export-delete-owner-proof.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/06-billing-owner-proof.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/16-validation-commands.log`.
- [x] Workpack completion section filled. Proof: `docs/plans/account-identity-family-plan/workpacks/05-device-ownership-authz.md`.

## WP07 Parent Account Family Setup UI

- [x] First-run UI state machine defined. Proof: `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/00-first-run-ui-state-machine.md`.
- [x] Sign-in/no-household/create-household/join-household states covered. Proof: `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/00-first-run-ui-state-machine.md`; `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/01-household-setup-ui-proof.md`.
- [x] Add child profile flow covered. Proof: `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/01-household-setup-ui-proof.md`; `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/05-mobile-parent-child-claim-split-proof.md`.
- [x] Pair child device flow covered. Proof: `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/02-device-role-ui-proof.md`.
- [x] Co-parent invite flow covered. Proof: `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/03-observer-read-only-ui-proof.md`.
- [x] Observer invite flow covered. Proof: `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/03-observer-read-only-ui-proof.md`.
- [x] Role visibility proof exists. Proof: `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/03-observer-read-only-ui-proof.md`; `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/05-mobile-parent-child-claim-split-proof.md`.
- [x] Device trust/revoked/expired-session status proof exists. Proof: `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/02-device-role-ui-proof.md`; `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/04-recovery-ui-proof.md`.
- [x] Recovery/support/manual-required states visible. Proof: `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/03-observer-read-only-ui-proof.md`; `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/04-recovery-ui-proof.md`.
- [x] Source/custody labels visible and honest. Proof: `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/06-source-custody-label-proof.md`.
- [x] UI does not imply login equals device trust. Proof: `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/02-device-role-ui-proof.md`; `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/05-mobile-parent-child-claim-split-proof.md`.
- [x] Portal tests or exact missing test blocker recorded. Proof: `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/16-validation-commands.log`.
- [x] Workpack completion section filled. Proof: `docs/plans/account-identity-family-plan/workpacks/07-parent-account-family-setup-ui.md`.

## WP06 Security Proof And Route Gate

- [x] WP01 proof root consumed. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/00-security-proof-pack.md`.
- [ ] Green WP08 Rust schema/account-authority proof root consumed; a recorded blocker leaves this gate and dependent scheduling blocked. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/09-account-authority-cloudflare-storage-gate.md`.
- [ ] Green Cloudflare WP06 storage proof and Cloudflare WP08 runner/proof are re-aggregated; any exact blocker is recorded without reusing prior WP06 completion evidence or releasing dependent scheduling. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/09-account-authority-cloudflare-storage-gate.md`.
- [x] WP02 proof root consumed. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/00-security-proof-pack.md`.
- [x] WP03 proof root consumed. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/00-security-proof-pack.md`; `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/05-origin-csrf-open-redirect-proof.md`.
- [x] WP04 proof root consumed. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/00-security-proof-pack.md`.
- [x] WP05 proof root consumed. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/00-security-proof-pack.md`.
- [x] WP07 proof root consumed or explicit UI blocker recorded. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/00-security-proof-pack.md`; `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/06-route-sync-proof.md`.
- [x] Authentication negative proof exists. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/01-authn-negative-proof.md`.
- [x] Authorization matrix proof exists. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/02-authz-matrix-proof.md`.
- [x] Token misuse proof exists. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/03-token-replay-proof.md`.
- [x] Recovery/invite misuse proof exists. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/04-recovery-abuse-proof.md`.
- [x] Origin and state-changing request safety proof exists or blocker recorded. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/05-origin-csrf-open-redirect-proof.md`.
- [x] Logging redaction proof exists. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/07-logging-redaction-proof.md`.
- [x] Route sync proof names setup, Cloudflare, payment, policy, data custody, device trust, LAN, and remote boundaries. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/06-route-sync-proof.md`.
- [x] Manual-required gap register written. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/08-manual-required-gap-register.md`.
- [ ] Focused validation commands are rerun after WP08 input and pass or blockers recorded. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/16-validation-commands.log`.
- [ ] Workpack completion section is re-filled only after the Account WP08 and Cloudflare WP06/WP08 final-gate inputs are aggregated. Proof: `docs/plans/account-identity-family-plan/workpacks/06-security-proof-and-route-gate.md`.

## 2026-08-17 live-code completion overlay

Historical checked WP02 rows retain their prior contract/proof meaning only.
They do not close the reviewed production gap:

- [ ] Target child/device is distinct from the actor parent-controller device
  for Pair, Register, Revoke, View, ChangePolicy, and Remote actions.
- [ ] Target-aware resolver derives current household/member/role/device state
  through the sealed authority boundary; callers cannot select authority facts.
- [ ] Capability, controller lease, and step-up requirements bind to the target
  action and are denied when stale, revoked, cross-household, or mismatched.
- [ ] Provider-to-authority production caller exists and is covered by focused
  expected tests; raw evaluator remains diagnostic-only or is retired safely.
- [ ] Focused tests, proof, and route-gate aggregation are rerun after the
  bounded source correction.

No WP02 completion or downstream Data custody readiness is claimed by these
unchecked rows.
