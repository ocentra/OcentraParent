#!/usr/bin/env node

import { failWithBlocker } from "./manual-required.js";

failWithBlocker(
  "infra/cloudflare/scripts/seed-products-local.ts",
  "product-seed-flow-not-implemented",
  "Implement local product and plan fixtures before using product seeding.",
);
