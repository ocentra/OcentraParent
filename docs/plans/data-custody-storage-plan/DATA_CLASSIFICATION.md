<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `DATA_CLASSIFICATION.md`
> Kind: data classification matrix.
> Read when: When a workpack needs a source-of-truth table for every data class.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Classes listed here must line up with bundle, event, sync, retention, and UI docs.

<!-- /agent-capsule -->

# Data Custody Storage Plan Data Classification

## Allowed Ocentra-hosted metadata

| Data class | Source of truth | Ocentra-hosted by default | Encrypted before upload | May appear in reports or notifications | Notes |
| --- | --- | --- | --- | --- | --- |
| Account identity metadata | Control plane / account plan | Yes | Yes, when payload leaves control plane | Yes, redacted | Identity, household, and entitlement metadata only |
| Subscription, billing, and entitlement metadata | Billing / payment plan | Yes | Yes where exported | Yes, redacted | Do not mix with child evidence |
| Device registration and pairing route metadata | Household control plane | Yes | Yes when persisted off-device | Yes, minimal | Route metadata only |
| Minimal notification routing metadata | Notification service | Yes | Yes for payload sections | Yes, minimal | Payload must stay redacted |
| Short-lived report compiler status | Report runtime / control plane | Yes | Not normally needed | Yes, minimal | Status only, not report content |
| Support case metadata | Support system | Yes | Yes when leaving trust boundary | Yes, redacted | No raw child activity by default |
| Public website and release status | Public site / release process | Yes | Not applicable | Yes | Public product metadata only |

## Household-owned or device-owned data classes

| Data class | Source of truth | Ocentra-hosted by default | Encrypted before upload | May appear in reports or notifications | Notes |
| --- | --- | --- | --- | --- | --- |
| Child profile | Child device / household model | No | Yes | Yes, only allowed references | Household-owned and role-bound |
| Parent rules and approval history | Household control plane | No | Yes | Yes, only allowed references | Source of truth for policy decisions |
| Evidence journal segments | Child device local journal | No | Yes | Yes, via references only | Local source of truth |
| SQLite evidence/read-model database | Child device local cache | No | Yes if exported | No | Rebuildable cache, not the truth layer |
| Screenshots and screen-analysis images | Child device local evidence | No | Yes | No unless explicitly exported | Sensitive and high-risk |
| Browser URL history | Child device local evidence | No | Yes | No unless explicitly exported | Never treated as generic telemetry |
| Network/app/game/tracking evidence | Child device local evidence | No | Yes | No unless explicitly exported | Same custody rules as screenshots |
| Generated long-term reports | Parent-owned output | No by default | Yes | Yes | Derived output, not source truth |
| Assistant child-evidence context | Parent assistant runtime | No by default | Yes | No raw content | Derived and limited |
| Parent-owned storage contents | Parent-selected provider | No by default | Yes | Yes, only metadata | Hidden app backup or visible folder modes differ |
| Provider sync payloads | Provider bundle | No by default | Yes | No raw content | Provider sees metadata only where unavoidable |
| Support bundles containing raw child activity | Support flow | No by default | Yes | No raw content | Support-safe redaction required |
| Universal decrypt keys | Household key model | No | N/A | No | Never host by default |

## Must never be hosted by default

- Raw child evidence.
- Encrypted journal segments without a household decision.
- SQLite evidence/read-model databases as a hosted truth layer.
- Screenshots or screen-analysis images.
- Browser URL history.
- Network/app/game/tracking evidence.
- Parent rules and approval history as an Ocentra-owned source of truth.
- Generated long-term reports as an Ocentra-owned truth layer.
- Assistant child-evidence context.
- Parent-owned storage contents.
- Provider sync payloads.
- Support bundles containing raw child activity.
- Universal decrypt keys.

## Data class completeness reminder

If a persisted thing is not listed above, it still needs a classification before the plan can call its custody model complete.

## Classification rules

- If the data is a source of truth, keep its owner explicit.
- If the data is derived, mark the original source and the allowed derived use.
- If the data can cross a custody boundary, it must be encrypted and classified before export or sync.
- If the data is merely routing or status metadata, keep it separate from payload content.
- If the data is a report, notification, or assistant context, it must cite or reference allowed source data only.
