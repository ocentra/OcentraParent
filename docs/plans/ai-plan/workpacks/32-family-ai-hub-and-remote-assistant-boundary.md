# 32 - Household AI Provider Mesh And Remote Assistant Boundary

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `32 - Household AI Provider Mesh And Remote Assistant Boundary`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: provider-mesh/remote-assistant boundary only after focused contract, custody, and negative proof exist.
> Does not prove: LAN transport readiness, remote access readiness, local model readiness, child safety product readiness, policy readiness, enforcement readiness, PR readiness, or broad DONE.
> Proof rule: Before DONE, apply `workpacks/00-owner-boundary-proof-gate.md`, select tests in TEST_PROOF_EXPECTATIONS.md, and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Household AI Provider Mesh is local/LAN child-safety compute sharing with child-agent authority. Remote Parent Assistant is parent-approved explanation/report help outside normal child safety. These are separate systems and must not be merged under one vague hub concept.

## Where We Are

Parent assistant provider routing proof exists. Legacy screen household-provider route proof exists for redacted/cropped local LAN custody, but the mesh does not yet prove provider discovery, claim/lease, result validation, child-agent policy authority, mobile dormant/fallback, or physical household LAN execution. Remote/API AI is already bounded by expectations but needs UI, custody, retention, and citation proof.

## Owner Path

```text
schema-domain:
  canonical provider advertisement, capability, job, claim, lease, result, validation, custody, citation, and remote-assistant request/result shapes when shared.
child-ai-core:
  child-agent result validation and accepted/rejected AI result boundary when selected.
lan-plan:
  physical LAN discovery/transport execution owner.
remote-access-plan:
  remote transport/access execution owner.
portal-domain/apps/portal:
  parent-visible status/approval/citation projection only.
data-custody-storage-plan:
  custody/retention policy owner when remote/report storage is selected.
```

Do not place LAN transport, remote access, custody storage, or policy/enforcement behavior in this workpack. This workpack defines and proves the boundary/handoff only unless explicitly assigned a narrower implementation slice.

## Checklist

- [ ] Separate household LAN provider mesh from remote assistant.
- [ ] Define provider as worker-only.
- [ ] Define child-agent policy authority.
- [ ] Define parent desktop AI provider role.
- [ ] Define child desktop AI provider role.
- [ ] Define mobile dormant/fallback role.
- [ ] Define remote/API assistant as report/explanation only.
- [ ] Require parent approval for remote/API use.
- [ ] Add data custody and retention state.
- [ ] Require evidence citations.
- [ ] Degrade to local-only explanation on failure.

## Proof

Required proof root:

```text
output/ai-plan-proof/32-family-ai-hub-and-remote-assistant-boundary/
```

Required proof:

- LAN provider worker-only proof.
- Provider discovery/claim/lease/result-validation proof or explicit LAN-owner blocker.
- Remote disabled-by-default test.
- Parent approval required test.
- Citation required test.
- Custody/retention boundary proof.
- Portal remote boundary screenshot or explicit missing-UI blocker.
- Mesh provider status screenshot or explicit missing-runtime blocker.

## Negative Cases

- Remote assistant cannot enter the normal child-safety blocking path.
- Remote/API use without parent approval is rejected or marked unavailable.
- Remote result without cited evidence is rejected.
- Provider worker result cannot become policy authority without child-agent validation.
- LAN provider discovery proof cannot claim remote access readiness.
- Parent desktop provider status cannot claim child-device policy authority.
- Mobile dormant/fallback state cannot silently drop required safety work.

## No-Claim Boundary

This workpack can prove the boundary and selected contracts/status surfaces only. It does not prove LAN transport, remote access transport, local model packaging, policy execution, enforcement execution, custody storage, or whole AI product readiness.
