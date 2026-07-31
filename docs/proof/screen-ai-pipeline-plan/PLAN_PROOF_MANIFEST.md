# Screen AI Pipeline Proof Manifest

## WP01 prerequisite merge gate

- Workpack: `01-prerequisite-merge-and-branch-gate`
- Proof command: `node scripts/test/screen-ai-prerequisite-merge-proof.mjs`
- Retained local artifacts: `output/screen-ai-pipeline-proof/prerequisite-merge/`
- Screen prerequisite: PR #574, `47151efa7ad617c1b0e8bd58ad499731fe9921ff`
- AI prerequisite: PR #455, `d85ab7c8ff90bce792b96150e6b7a0b7ade5fa00`
- Required current-head Rust surfaces: `screen-core`, `screen-ai-core`,
  `agent-service`, `ocentra-eventing`, and `schema`.
- Negative proof: an unknown prerequisite commit must cause the harness to fail
  before it writes a passing summary.
- No claim: this confirms prerequisite provenance and availability only. It
  does not prove the trigger-to-capture, capture-to-analysis, policy/action,
  journal/portal, custody, live-operator, or rollout gates.
