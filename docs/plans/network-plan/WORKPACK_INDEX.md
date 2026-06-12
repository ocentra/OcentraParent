# Network Plan Workpack Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Plan Workpack Index`
> Kind: workpack selector; use before opening any workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Choose one row, then open only that workpack plus exact checklist/proof/test rows it names.
> Proves: local network workpack routing only.
> Does not prove: live capture, privileged adapter execution, product enforcement, mobile authority, PR readiness, or broad DONE.
> Proof rule: If this index changes status, update `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `PLAN_HEALTH.md`, and affected checklist/proof rows.

<!-- /agent-capsule -->

Use this file to select exactly one network slice. Do not read all network docs, the full checklist, or the large control inventories by default.

Source truth for historical scope is [03-network-implementation-checklist-and-workpacks.md](03-network-implementation-checklist-and-workpacks.md). The route workpacks below split that scope into high-density execution paths.

| Status | Workpack                                                                                            | Source rows       | Required proof tier | Open condition                                                                                                                    |
| ------ | --------------------------------------------------------------------------------------------------- | ----------------- | ------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| open   | [01 Foundation Contracts And Eventing](workpacks/01-foundation-contracts-and-eventing.md)           | 1-10              | P0_CONTRACT         | Until network evidence schemas, Rust parity, evidence grade, policy action, and eventing integration are proved.                  |
| open   | [02 Passive Capture And Parsing](workpacks/02-passive-capture-and-parsing.md)                       | 11-20             | P1/P3 by claim      | Until PCAP replay, parser fixtures, live-capture limits, and no-overclaim guards exist.                                           |
| open   | [03 Classification And Correlation](workpacks/03-classification-and-correlation.md)                 | 21-30             | P1/P2               | Until domain/category, tunnel, remote desktop, browser/app/screen correlation, and ambiguity proof exists.                        |
| open   | [04 Cross Slice Cascade And Parent Surface](workpacks/04-cross-slice-cascade-and-parent-surface.md) | 31-36             | P2/P3               | Until cascade, bundle, AI queue, policy mapping, notification candidate, and parent UI drawer proof exists.                       |
| open   | [05 Intervention Adapter Proof Gates](workpacks/05-intervention-adapter-proof-gates.md)             | 37-42             | P3/P5 by platform   | Until DNS, firewall, WFP, Android VPN, Apple Network Extension, and Linux mechanism proof gates are satisfied or manual-required. |
| open   | [06 Analyzer AI Audit And Risk Budget](workpacks/06-analyzer-ai-audit-and-risk-budget.md)           | 43-48             | P2/P3               | Until Zeek-style, Suricata/Snort-compatible, AI evaluation, audit narrative, and risk budget proof exists.                        |
| open   | [07 Performance Security Rollout](workpacks/07-performance-security-rollout.md)                     | 49-50             | P2/P6 by claim      | Until concurrency, resource, privacy, compliance, deployment, support, and staged rollout proof exists.                           |
| open   | [08 Control Catalog Reference Routing](workpacks/08-control-catalog-reference-routing.md)           | moved source docs | route/reference     | Until giant capability/schema/settings docs are only opened by exact selected control work.                                       |

## Selection Rules

- If the task names a numbered row from `03-network-implementation-checklist-and-workpacks.md`, choose the workpack whose source-row range contains it.
- If the task is about a root-level moved control guide, choose WP08 first, then the precise referenced file.
- If the task implies exact URL/video/message/search content from network-only evidence, stop and record a failure condition.
- If platform intervention is touched, choose WP05 and require the platform proof tier before claiming enforcement.
