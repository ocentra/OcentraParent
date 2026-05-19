---
name: ocentra-parent-rule-router
description: Choose and read the focused Ocentra Parent rule files before coding. Use when starting a task, changing architecture, or touching multiple repo layers.
---

# Ocentra Parent Rule Router

Use this skill before making code changes in Ocentra Parent.

## Workflow

1. Read `.ocentra-ai/rules/ocentra-parent-rules.mdc`.
2. Identify the files or packages likely to change.
3. Read every routed rule file that matches those paths.
4. State the applicable rule files briefly before editing.
5. Make the smallest change that satisfies the task and the rules.
6. Run the validation gate appropriate to the touched layer.

## Routing Shortcuts

- Tests: read `ocentra-parent-test-rules.mdc`.
- Protocol: read `ocentra-parent-protocol-websocket.mdc`.
- Rust service: read `ocentra-parent-rust-service.mdc`.
- Portal: read `ocentra-parent-portal.mdc`.
- Logs: read `ocentra-parent-logging-redaction.mdc`.
- Security: read `ocentra-parent-security-localhost.mdc`.
- Any source edit: read `ocentra-parent-domain-boundaries.mdc`, `ocentra-parent-source-shape.mdc`, and `ocentra-parent-validation.mdc`.
