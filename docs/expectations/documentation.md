<!-- agent-capsule -->

> Agent Capsule
> Doc: Documentation Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Documentation Expectations

Docs must change when the product claim changes.

Documentation is part of the product contract. Product-facing docs explain why a
parent would choose Ocentra. Expectation docs define what must be true. Module
READMEs explain where the work belongs. Checkpoints record proof; they do not
replace the constitution, roadmap, or capability checklist.

## Update Docs When Changing

- Root README user-facing positioning.
- [Product constitution](../product-constitution.md) when product truths or
  status language change.
- [Product capability checklist](../product-capability-checklist.md) when a
  feature moves status, gains proof, or gets a new gap.
- [Feature list](../feature-list.md) and its per-feature docs when a feature
  changes status, scope, competitor gap, roadmap placement, or checklist.
- [Competitor capability map](../competitor-capability-map.md) when parity
  requirements or competitor baselines change.
- Roadmap status.
- Architecture boundaries.
- Public intents or contracts.
- Storage model.
- Platform support claims.
- Installer/update behavior.
- Security/privacy posture.
- Validation gates.
- Parent-visible behavior.
- Module README ownership and gap notes when an app, package, crate, or platform
  area changes responsibility.
- `production-release-public-docs-status-proof` may prove source-contract
  status for public privacy, retention, export/delete, support, incident, and
  legal docs, but docs must not claim public publication until a public route,
  legal review, and publication proof exist.
- `production-release-public-surface-publication-proof` may prove a composed
  publication/readiness summary for `family.ocentra.ca` public status, runtime
  handoff, and public docs rows, but docs must still label public runtime,
  account backend, billing provider runtime, signing/store proof, updater
  execution, support upload, production SLA, legal execution, and child-activity
  custody as gaps until real evidence exists.
- `public-support-contact-status-proof` may prove public support contact/status
  source-contract rows, but docs must still label public runtime execution,
  support backend upload execution, account lookup execution, billing provider
  contact, remote support sessions, production SLA, legal disclosure execution,
  provider secrets, and child-activity custody as gaps until real evidence
  exists.
- `production-support-publication-status-freshness-proof` may prove source
  contract freshness for support runbook, incident status, public support
  contact, support backend upload publication, privacy/legal publication, and
  account/billing support publication rows, but docs must still label public
  runtime execution, support publication execution, support backend upload
  execution, account lookup execution, billing provider contact, remote support
  sessions, production SLA, legal disclosure execution, provider secrets, and
  child-activity custody as gaps until real evidence exists.
- `production-support-legal-provider-readiness-proof` may prove
  privacy/legal-review, export/delete runtime, provider-secret custody,
  billing-provider contact, remote-support legal/session, and production SLA
  legal-boundary readiness rows, but docs must still label real legal
  disclosure execution, export/delete runtime execution, provider secret
  custody, billing provider contact execution, account lookup execution, remote
  support sessions, production SLA commitments, support backend upload
  execution, public runtime execution, and child-activity custody as gaps until
  real evidence exists.
- `production-support-data-export-delete-lifecycle-proof` may prove
  requested, authorized, queued, running, succeeded, failed, and
  manual-required export/delete lifecycle status rows for parent-authorized
  local runtime proof, but docs must still label real backend upload
  execution, public runtime execution, provider execution, production SLA,
  remote support sessions, default Ocentra-hosted family data, child-activity
  custody, durable production queues, and delete executor proof as gaps until
  real evidence exists.
- `production-support-backend-upload-status-proof` may prove support upload
  status/read-model rows, redaction/audit refs, retry/abandon refs, and manual
  proof requirements, but docs must still label raw child activity custody,
  provider secrets, remote support transcripts, real backend upload execution,
  account lookup execution, billing provider execution, default Ocentra-hosted
  family data, and production SLA as gaps until real evidence exists.
- `production-support-publication-runtime-readiness-proof` may prove
  source-backed publication/runtime readiness rows for public runtime, support
  runbook publication runner, incident status publication runner, support upload
  publication runtime, privacy/legal publication runtime, and public support
  contact runtime handoffs, but docs must still label real public runtime
  execution, publication runner execution, support backend upload execution,
  account lookup execution, billing provider contact execution, remote support
  sessions, legal disclosure execution, provider secrets, raw child activity
  custody, default Ocentra-hosted family data, and production SLA as gaps until
  real evidence exists.
- `production-support-process-runtime-status-proof` may prove support process
  requested, parent-consent authorized, privacy/legal queued, redaction review
  running, backend-upload failed, case-resolution succeeded, and
  manual-required runtime status rows, but docs must still label real backend
  upload execution, public runtime execution, provider execution, production
  SLA, remote support sessions, provider secrets, child activity custody, and
  default Ocentra-hosted family data as gaps until real evidence exists.
- `production-support-publication-execution-status-proof` may prove status
  contract rows for support/publication execution targets across requested,
  queued, running, succeeded, failed, and manual-required states, but docs must
  still label real public runtime execution, publication runner execution,
  status backend execution, support backend upload execution, account lookup
  execution, billing provider contact execution, remote support sessions, legal
  disclosure execution, provider secrets, raw child activity custody, default
  Ocentra-hosted family data, and production SLA as gaps until real evidence
  exists.

## Required Structure

Every feature-facing documentation update should leave these files consistent:

- `README.md`: user-facing product promise plus honest repo status links.
- `docs/product-constitution.md`: product truths, status vocabulary, claim gate.
- `docs/product-roadmap.md`: milestone order and acceptance links.
- `docs/product-capability-checklist.md`: current status, proof, and next gap.
- `docs/feature-list.md` and `docs/features/*.md`: per-feature expectations,
  competitor pressure, gaps, and checklists.
- `docs/feature-expectations.md`: expectation index.
- relevant `docs/expectations/*.md`: acceptance contract.
- relevant module README: ownership, flow, gaps, and connected docs.

## AI Agent Reading Rule

Agents should read docs by task, not by folder:

1. Start with `docs/feature-list.md`.
2. Open the one owning `docs/features/*.md` file.
3. Open only the linked expectation docs that apply to the task.
4. Open the relevant milestone section of `docs/product-roadmap.md` only when
   milestone scope or status may change.
5. Open the relevant `docs/product-capability-checklist.md` rows before and
   after status/proof/gap changes.
6. Open module READMEs only for touched modules.

Do not bulk-read all feature docs, expectation docs, or checkpoint records.
Checkpoints are proof records, not routing docs.

## Docs Should Not

- Claim future features as implemented.
- Hide uncertainty.
- Use marketing wording where a technical limitation matters.
- Duplicate contract truth when a domain package owns the real schema.
- Let historical checkpoint wording override current roadmap/checklist status.
- Treat a scaffold, package preview, or UI shell as product completion.
- Say "done" without naming platform scope and proof.

## Done Signal

The docs make it clear what is implemented, what is scaffold-only, what remains
intentionally out of scope, what is a tracked gap, which expectation files apply,
which module owns the behavior, and what proof is required before the product
claim can be called done.
