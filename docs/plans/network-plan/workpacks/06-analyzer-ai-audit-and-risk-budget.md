# WP06 Analyzer AI Audit And Risk Budget

Scope: prove structured analyzer inputs, AI detection evaluation, audit narrative, and household risk budget without unsupported content claims.

Source rows: `03-network-implementation-checklist-and-workpacks.md` rows 43-48.

Read next:

- `../02-network-tests-proof-and-validation-blueprint.md`
- `../../ai-plan/AGENTS.md`
- `../../policy-control-plane-plan/AGENTS.md` only when risk budget influences policy

Expected outcome:

- Zeek-style structured summaries and Suricata/Snort-compatible signature alerts are treated as evidence inputs with provenance.
- AI detection model evaluation uses structured summaries and evidence refs, not raw family packet content.
- AI audit narrative explains confidence, uncertainty, missing signals, and no unsupported exact-content claims.
- Household risk budget and cascade thresholds are typed, tested, and policy-gated.

Expected tests/proof:

- `network.analyzer.zeek-summary-fixtures`
- `network.analyzer.signature-alert-ingestion`
- `network.ai-detection.fixture-eval`
- `network.ai-audit.no-exact-content-claim`
- `network.risk-budget.threshold-boundary`
- `network.risk-budget.policy-gate`
- Proof includes model/eval fixture names, drift/precision notes, and false-positive handling.

Failure conditions:

- Do not treat analyzer alerts as enforcement authority.
- Do not let AI narrative claim exact content, exact URL, exact video, or private messages.
- Do not let risk budget affect action without policy-control and enforcement proof.
