# Local Dev And Seeding Model

Purpose: define the first local worker workflow before payment runtime work starts.

## Local development contract

- Default command: `npm --prefix infra/cloudflare run dev`
- Mode: `wrangler dev --local`
- Expected local caller: parent portal or local smoke tooling
- Required local docs: origins, seed order, teardown, and blocker reporting

## Seed commands

- `seed:local`
- `seed:products:local`
- `seed:referrals:local`
- `seed:test-accounts:local`

## Required fixture families

- pricing catalog
- parent test accounts
- referral test graph
- support/admin test accounts
- webhook payload fixtures
- queue replay fixtures

## Manual-required states

- If local D1 or Queue emulation is not wired yet, record the exact blocker.
- If account auth adapter is unresolved, local auth may use a stub only with a clear `manual-required` note.
