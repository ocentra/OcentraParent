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

## WP02 Identity Household Role Model

- [ ] Account user model defined.
- [ ] Household model defined.
- [ ] Membership state machine defined.
- [ ] Role/action/resource matrix defined.
- [ ] Child profile and child device separated.
- [ ] Parent owner/co-parent/observer roles separated.
- [ ] Support/admin actor minimized and audited.
- [ ] Pending/invited/revoked/disabled states represented.
- [ ] Cross-family negative proof exists.
- [ ] Observer read-only proof exists.
- [ ] Audit-event requirement proof exists.
- [ ] Required proof artifacts written.
- [ ] Workpack completion section filled.

## WP03 Session Token Lifecycle

- [ ] Credential type matrix defined. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/00-credential-type-matrix.md`.
- [ ] Browser session lifecycle defined. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/01-session-lifecycle-proof.md`.
- [ ] Refresh rotation defined. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/03-refresh-revocation-proof.md`.
- [ ] Logout/global revoke behavior defined. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/01-session-lifecycle-proof.md`.
- [ ] Expiry and clock-skew behavior defined. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/02-token-expiry-replay-proof.md`.
- [ ] Reuse and stolen-token negative proof exists. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/02-token-expiry-replay-proof.md`.
- [ ] Device credential separated from browser session. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/00-credential-type-matrix.md`.
- [ ] Invite/recovery/controller-lease tokens separated from sessions. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/00-credential-type-matrix.md`.
- [ ] Freshness requirement defined for sensitive actions. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/04-session-freshness-proof.md`.
- [ ] State-changing request safety proof or blocker exists. Blocker: `output/account-identity-family-plan-proof/03-session-token-lifecycle/05-csrf-origin-proof.md`.
- [ ] Redacted audit log proof exists. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/06-token-redaction-proof.md`.
- [ ] Required proof artifacts written. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/00-credential-type-matrix.md`, `output/account-identity-family-plan-proof/03-session-token-lifecycle/01-session-lifecycle-proof.md`, `output/account-identity-family-plan-proof/03-session-token-lifecycle/02-token-expiry-replay-proof.md`, `output/account-identity-family-plan-proof/03-session-token-lifecycle/03-refresh-revocation-proof.md`, `output/account-identity-family-plan-proof/03-session-token-lifecycle/04-session-freshness-proof.md`, `output/account-identity-family-plan-proof/03-session-token-lifecycle/05-csrf-origin-proof.md`, `output/account-identity-family-plan-proof/03-session-token-lifecycle/06-token-redaction-proof.md`, `output/account-identity-family-plan-proof/03-session-token-lifecycle/16-validation-commands.log`.
- [ ] Focused commands pass or blocker recorded. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/16-validation-commands.log`.
- [ ] Workpack completion section filled. Proof: `docs/plans/account-identity-family-plan/workpacks/03-session-token-lifecycle.md`.

## WP04 Invites Recovery Lifecycle

- [ ] Invite state machine defined. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/00-invite-state-machine-proof.md`.
- [ ] Co-parent/observer/child-device invite scopes separated. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/00-invite-state-machine-proof.md`.
- [ ] Invite single-use proof exists. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/00-invite-state-machine-proof.md`.
- [ ] Expired/revoked/reused invite negative proof exists. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/01-invite-negative-proof.md`.
- [ ] Wrong-household/wrong-role invite negative proof exists. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/01-invite-negative-proof.md`.
- [ ] Recovery state machine defined. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/02-recovery-state-machine-proof.md`.
- [ ] Forgotten-login recovery modeled. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/02-recovery-state-machine-proof.md`.
- [ ] Lost-parent-device and compromised-account recovery modeled. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/02-recovery-state-machine-proof.md`.
- [ ] Child reinstall and household transfer modeled. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/02-recovery-state-machine-proof.md`.
- [ ] Recovery rate-limit/enumeration-resistant proof exists. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/03-recovery-abuse-proof.md`.
- [ ] Delete/export handoff to data custody documented. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/04-delete-export-handoff-proof.md`.
- [ ] Support recovery audit proof exists. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/05-support-recovery-audit-proof.md`.
- [ ] Workpack completion section filled. Proof: `docs/plans/account-identity-family-plan/workpacks/04-invites-recovery-lifecycle.md`.

## WP05 Device Ownership AuthZ

- [ ] Actor/household/role/device/session/capability matrix defined. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/00-device-authority-matrix.md`.
- [ ] Parent controller authority proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/00-device-authority-matrix.md`.
- [ ] Parent observer read-only proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/04-remote-capability-proof.md`.
- [ ] Child agent authority proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/00-device-authority-matrix.md`.
- [ ] Pending/trusted/revoked/disabled/stale device states covered. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/01-revoked-device-negative-proof.md`.
- [ ] Wrong-household denial proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/02-wrong-household-negative-proof.md`.
- [ ] Controller lease required/expired/revoked proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/03-controller-lease-proof.md`.
- [ ] Remote view/control capability separation proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/04-remote-capability-proof.md`.
- [ ] Export/delete owner-only proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/05-export-delete-owner-proof.md`.
- [ ] Billing parent-owner proof exists. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/06-billing-owner-proof.md`.
- [ ] Audit-event requirement proof exists. Proof: `packages/family-domain/src/household-authority.ts`.
- [ ] Required proof artifacts written. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/00-device-authority-matrix.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/01-revoked-device-negative-proof.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/02-wrong-household-negative-proof.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/03-controller-lease-proof.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/04-remote-capability-proof.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/05-export-delete-owner-proof.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/06-billing-owner-proof.md`; `output/account-identity-family-plan-proof/05-device-ownership-authz/16-validation-commands.log`.
- [ ] Workpack completion section filled. Proof: `docs/plans/account-identity-family-plan/workpacks/05-device-ownership-authz.md`.

## WP07 Parent Account Family Setup UI

- [ ] First-run UI state machine defined.
- [ ] Sign-in/no-household/create-household/join-household states covered.
- [ ] Add child profile flow covered.
- [ ] Pair child device flow covered.
- [ ] Co-parent invite flow covered.
- [ ] Observer invite flow covered.
- [ ] Role visibility proof exists.
- [ ] Device trust/revoked/expired-session status proof exists.
- [ ] Recovery/support/manual-required states visible.
- [ ] Source/custody labels visible and honest.
- [ ] UI does not imply login equals device trust.
- [ ] Portal tests or exact missing test blocker recorded.
- [ ] Workpack completion section filled.

## WP06 Security Proof And Route Gate

- [ ] WP01 proof root consumed. Proof: `output/account-identity-family-plan-proof/01-auth-provider-decision/`.
- [ ] WP02 proof root consumed. Proof: `output/account-identity-family-plan-proof/02-identity-household-role-model/`.
- [ ] WP03 proof root consumed. Proof: `output/account-identity-family-plan-proof/03-session-token-lifecycle/`.
- [ ] WP04 proof root consumed. Proof: `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/`.
- [ ] WP05 proof root consumed. Proof: `output/account-identity-family-plan-proof/05-device-ownership-authz/`.
- [ ] WP07 proof root consumed or explicit UI blocker recorded. Proof: `docs/plans/account-identity-family-plan/workpacks/07-parent-account-family-setup-ui.md`.
- [ ] Authentication negative proof exists. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/01-authn-negative-proof.md`.
- [ ] Authorization matrix proof exists. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/02-authz-matrix-proof.md`.
- [ ] Token misuse proof exists. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/03-token-replay-proof.md`.
- [ ] Recovery/invite misuse proof exists. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/04-recovery-abuse-proof.md`.
- [ ] Origin and state-changing request safety proof exists or blocker recorded. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/05-origin-csrf-open-redirect-proof.md`.
- [ ] Logging redaction proof exists. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/07-logging-redaction-proof.md`.
- [ ] Route sync proof names setup, Cloudflare, payment, policy, data custody, device trust, LAN, and remote boundaries. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/06-route-sync-proof.md`.
- [ ] Manual-required gap register written. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/08-manual-required-gap-register.md`.
- [ ] Focused validation commands pass or blockers recorded. Proof: `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/16-validation-commands.log`.
- [ ] Workpack completion section filled. Proof: `docs/plans/account-identity-family-plan/workpacks/06-security-proof-and-route-gate.md`.
