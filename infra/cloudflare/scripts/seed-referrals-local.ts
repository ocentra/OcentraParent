#!/usr/bin/env node

import { failWithBlocker } from "./manual-required.js";

failWithBlocker(
  "infra/cloudflare/scripts/seed-referrals-local.ts",
  "referral-seed-flow-not-implemented",
  "Implement local referral fixtures and abuse-review cases before using referral seeding.",
);
