#!/usr/bin/env node

import { failWithBlocker } from "./manual-required.js";

failWithBlocker(
  "infra/cloudflare/scripts/seed-test-accounts-local.ts",
  "test-account-seed-flow-not-implemented",
  "Implement test account fixtures and teardown before using test-account seeding.",
);
