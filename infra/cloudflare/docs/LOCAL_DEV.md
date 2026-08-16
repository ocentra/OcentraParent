# Local Dev

Intended local commands:

- root: `npm run dev:cloudflare`
- module: `npm run dev`
- seeding: `npm run seed:local`

Current blocker:

- local test and seed helpers exist and are used by the scoped Cloudflare suites
- a dedicated proof bundle for start, seed, and teardown is still missing under `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/`
