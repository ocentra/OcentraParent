#!/usr/bin/env node

import { failWithBlocker } from "./manual-required.js";

failWithBlocker(
  "infra/cloudflare/scripts/seed-local.ts",
  "local-seed-flow-not-implemented",
  "Implement Wrangler-local billing fixtures, teardown, and proof before using shared Cloudflare seeding.",
);
