# WP01 rollback and teardown proof

plan: cloudflare-control-plane-plan
workpack: 01-cloudflare-module-scaffold
owner: infra-cloudflare
environment: local

Rollback is limited to restoring the prior manifest declaration and rerunning
the module install/graph command. No remote resource, Wrangler deployment,
binding, or secret was created or changed.

Validation installed only ignored local dependency directories under
`infra/cloudflare/node_modules` and `packages/logging-domain/node_modules`.
They are disposable local validation state; removing them does not change the
committed source or retained proof. Recreate them with the documented
ignore-scripts installs before rerunning module checks.

The local logging-domain build prerequisite writes ignored
`packages/logging-domain/dist`. After verifying that exact directory is not a
reparse point and was created for the validation pass, its teardown command is
`Remove-Item -LiteralPath packages/logging-domain/dist -Recurse -Force`.
