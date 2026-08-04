# WP01 module scaffold and package scripts

plan: cloudflare-control-plane-plan
workpack: 01-cloudflare-module-scaffold
owner: infra-cloudflare
environment: local
proof_ids: cloudflare-control.module-exists, cloudflare-control.package-scripts

Static scaffold audit result: pass.

The audited module surface contains the package manifest, development and
production Wrangler configurations, placeholder-only `.dev.vars.example`,
entrypoint/env/route source, the module test runner, and a representative unit
route-manifest test. The package manifest declares the explicit `dev`,
`dev:remote`, `seed:local`, unit/integration/e2e/contract/security/property/fuzz
test, deployment, and lint scripts.

```text
command: node -e "const fs=require('fs'); const p=require('./infra/cloudflare/package.json'); const files=['infra/cloudflare/package.json','infra/cloudflare/wrangler.toml','infra/cloudflare/wrangler.production.toml','infra/cloudflare/.dev.vars.example','infra/cloudflare/src/index.ts','infra/cloudflare/src/env.ts','infra/cloudflare/src/routes.ts','infra/cloudflare/scripts/test-runner.ts','infra/cloudflare/tests/unit/route-manifest.test.ts']; const scripts=['dev','dev:remote','seed:local','test:unit','test:integration','test:e2e','test:contract','test:security','test:property','test:fuzz','deploy:dev','deploy','lint']; const missing=files.filter((f)=>!fs.existsSync(f)); const missingScripts=scripts.filter((s)=>typeof p.scripts[s]!=='string'); if(missing.length||missingScripts.length){console.error(JSON.stringify({missing,missingScripts}));process.exit(1)} console.log(JSON.stringify({files:files.length,scripts:scripts.length,result:'pass'}));"
exit: 0
result: pass
notes: 9 paths and 13 explicit scripts verified
```

This is static module-tree and package-script evidence only. It does not prove
that a Worker boots, deployment succeeds, a binding exists, or a consumer flow
is runtime-ready.
