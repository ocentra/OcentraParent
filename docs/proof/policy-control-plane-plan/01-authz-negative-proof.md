# WP01 Authorization Negative Proof

## Proves

- `policy-source.authz-role-matrix`
- `policy-source.wrong-household-rejected`
- `policy-source.revoked-actor-rejected`

## Evidence

- `packages/policy-domain/tests/unit/authority.test.ts`
  - `allows only parent policy decisions to authorize enforcement handoff`
  - `keeps AI and tracking signals as evidence-only inputs`
  - `keeps dry-run parent policy decisions out of enforcement authority`
- `crates/policy-control-core/tests/unit/policy_source.rs`
  - `coparent_can_write_source_truth_but_child_and_support_cannot`
  - `wrong_household_actor_authority_cannot_register_source_truth`
  - `mismatched_actor_authority_cannot_register_source_truth`
  - `mismatched_role_authority_cannot_register_source_truth`
  - `revoked_actor_authority_cannot_register_source_truth`
- `crates/policy-control-core/tests/unit/policy_request.rs`
  - `child_and_support_roles_cannot_confirm_or_self_approve`
  - `observer_and_revoked_parent_cannot_confirm_or_approve`
  - `wrong_parent_household_cannot_approve_request`

## Result

- Wrong-household, wrong-role, revoked, child, support, and observer actors are denied at the contract boundary.
- Coparent authority is explicit and typed rather than inferred.
- Evidence-only and dry-run decisions do not acquire enforcement authority.

