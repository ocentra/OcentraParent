---
name: ocentra-parent-rule-router
description: Choose and read the focused Ocentra Parent rule files before coding. Use when starting a task, changing architecture, or touching multiple repo layers.
---

# Ocentra Parent Rule Router

Use this skill before making code changes in Ocentra Parent.

## Workflow

1. Read `.ocentra-ai/rules/ocentra-parent-rules.mdc`.
2. For product-facing work, read `docs/feature-list.md`, then read only the
   owning `docs/features/*.md` file for the task.
3. Read only the expectation files linked by that feature doc that apply to the
   files you will touch.
4. Identify the files or packages likely to change.
5. Read every routed rule file that matches those paths.
6. Read the README for each touched app/package/crate/platform area.
7. State the applicable feature doc, expectation docs, module README, and rule
   files briefly before editing.
8. Make the smallest change that satisfies the task and the rules.
9. If feature status/proof/gap changes, update the owning feature doc and
   `docs/product-capability-checklist.md`; update roadmap, expectation docs,
   module README, README, or competitor map only when their contracts changed.
10. Run the validation gate appropriate to the touched layer.

## Context Hygiene

- Do not load every feature, expectation, roadmap, or checkpoint document.
- Do not open historical checkpoint files unless the feature doc, checklist,
  roadmap, or hub assignment names them as current proof.
- If no feature doc owns the task, add or update one before making broad product
  claims.

## Routing Shortcuts

- Tests: read `ocentra-parent-test-rules.mdc`.
- Protocol: read `ocentra-parent-protocol-websocket.mdc`.
- Rust service: read `ocentra-parent-rust-service.mdc`.
- Portal: read `ocentra-parent-portal.mdc`.
- Logs: read `ocentra-parent-logging-redaction.mdc`.
- Security: read `ocentra-parent-security-localhost.mdc`.
- Any source edit: read `ocentra-parent-domain-boundaries.mdc`, `ocentra-parent-source-shape.mdc`, and `ocentra-parent-validation.mdc`.
