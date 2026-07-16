# Deployment

Current deployment shape:

- `wrangler.toml` is the development config with explicit binding names and placeholder resource IDs
- `wrangler.production.toml` is the production config with explicit binding names and placeholder resource IDs
- both files still require real environment ownership and promotion proof

Current blocker:

- no deploy proof exists
- do not treat config presence as promotion readiness
