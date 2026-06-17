# WP01 AI Preview Not Write Proof

## Proves

- `policy-source.ai-preview-not-write`
- `policy-source.portal-ui-not-truth`

## Evidence

- `packages/policy-domain/tests/unit/policy.test.ts`
  - `parsePolicyPreview: keeps assistant-authored previews confirmation-required until a parent confirms`
  - `parsePolicyPreview: rejects preview decisions that attempt runtime handoff`
- `packages/policy-domain/tests/unit/policy-approval-override.test.ts`
  - `resolvePolicyApprovalLifecycle: keeps assistant-drafted actions preview-only until a parent confirms`
- `crates/policy-control-core/tests/unit/policy_source.rs`
  - `ai_preview_and_domain_cache_cannot_become_source_truth`
- `crates/policy-control-core/tests/unit/policy_request.rs`
  - `assistant_draft_stays_preview_only_until_parent_confirms`

## Result

- AI- and portal-origin previews remain preview-only artifacts until parent confirmation.
- Preview objects cannot jump directly to runtime handoff or canonical source truth.

## Does not prove

- Assistant chat/portal UX completion.
- Child-agent validation after confirmation.

