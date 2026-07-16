# Testing

Required test families:

- unit
- integration
- e2e
- contract
- security
- property
- fuzz

Current state:

- runnable test files exist for each family
- `scripts/test-runner.ts` executes real scoped suites through `tsx`
- local Wrangler-backed runtime coverage exists inside the integration family
- passing claims still require scoped command logs and proof artifacts under `output/cloudflare-control-plane-plan-proof/`
