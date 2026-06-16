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

- [ ] Source-backed provider decision record written.
- [ ] Cloudflare custody boundary accepted or blocker recorded.
- [ ] Firebase/Auth.js/other provider role accepted, rejected, or staged.
- [ ] Custom claims data-minimization rule documented.
- [ ] Identity provider cannot own household/product data.
- [ ] Provider outage/degraded state documented.
- [ ] Migration/replaceability path documented.
- [ ] Dev-mode bypass cannot reach production proof written.
- [ ] Required proof artifacts written.
- [ ] Workpack completion section filled.

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

- [ ] Credential type matrix defined.
- [ ] Browser session lifecycle defined.
- [ ] Refresh rotation defined.
- [ ] Logout/global revoke behavior defined.
- [ ] Expiry and clock-skew behavior defined.
- [ ] Reuse and stolen-token negative proof exists.
- [ ] Device credential separated from browser session.
- [ ] Invite/recovery/controller-lease tokens separated from sessions.
- [ ] Freshness requirement defined for sensitive actions.
- [ ] State-changing request safety proof or blocker exists.
- [ ] Redacted audit log proof exists.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.

## WP04 Invites Recovery Lifecycle

- [ ] Invite state machine defined.
- [ ] Co-parent/observer/child-device invite scopes separated.
- [ ] Invite single-use proof exists.
- [ ] Expired/revoked/reused invite negative proof exists.
- [ ] Wrong-household/wrong-role invite negative proof exists.
- [ ] Recovery state machine defined.
- [ ] Forgotten-login recovery modeled.
- [ ] Lost-parent-device and compromised-account recovery modeled.
- [ ] Child reinstall and household transfer modeled.
- [ ] Recovery rate-limit/enumeration-resistant proof exists.
- [ ] Delete/export handoff to data custody documented.
- [ ] Support recovery audit proof exists.
- [ ] Workpack completion section filled.

## WP05 Device Ownership AuthZ

- [ ] Actor/household/role/device/session/capability matrix defined.
- [ ] Parent controller authority proof exists.
- [ ] Parent observer read-only proof exists.
- [ ] Child agent authority proof exists.
- [ ] Pending/trusted/revoked/disabled/stale device states covered.
- [ ] Wrong-household denial proof exists.
- [ ] Controller lease required/expired/revoked proof exists.
- [ ] Remote view/control capability separation proof exists.
- [ ] Export/delete owner-only proof exists.
- [ ] Billing parent-owner proof exists.
- [ ] Audit-event requirement proof exists.
- [ ] Required proof artifacts written.
- [ ] Workpack completion section filled.

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

- [ ] WP01 proof root consumed.
- [ ] WP02 proof root consumed.
- [ ] WP03 proof root consumed.
- [ ] WP04 proof root consumed.
- [ ] WP05 proof root consumed.
- [ ] WP07 proof root consumed or explicit UI blocker recorded.
- [ ] Authentication negative proof exists.
- [ ] Authorization matrix proof exists.
- [ ] Token misuse proof exists.
- [ ] Recovery/invite misuse proof exists.
- [ ] Origin and state-changing request safety proof exists or blocker recorded.
- [ ] Logging redaction proof exists.
- [ ] Route sync proof names setup, Cloudflare, payment, policy, data custody, device trust, LAN, and remote boundaries.
- [ ] Manual-required gap register written.
- [ ] Focused validation commands pass or blockers recorded.
- [ ] Workpack completion section filled.
