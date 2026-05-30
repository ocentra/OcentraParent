# Local AI Runtime Provider Proof - 2026-05-30

Branch: `codex/local-ai-runtime-provider-proof`

Scope:

- one local `ai-provider` role per physical device
- shared parent/child same-device provider contract
- one local model runtime lane per physical device
- child-safety scheduler priority over parent-assistant work
- queued, degraded, and unavailable provider lifecycle states
- parent-assistant local-provider submission boundary
- duplicate local model load refusal

Proof command:

```powershell
node scripts/test/local-ai-runtime-provider-proof.mjs
```

Expected generated evidence:

```text
test-results/local-ai-runtime-provider-proof/proof.json
```

Product truth boundary:

- This proves same-device local provider/scheduler contracts and service state.
- This does not prove LAN AI provider pooling, cross-device AI job routing, remote/API provider authorization, model quality, or child-safety classifier accuracy.
- Portal remains a read/render surface for this proof; runtime ownership stays in the Rust service/child-agent path.
