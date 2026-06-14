#!/usr/bin/env node

import { readdirSync } from "node:fs";
import { resolve } from "node:path";
import { failWithBlocker } from "./manual-required.js";

const FAMILY_DIRS = {
  unit: "tests/unit",
  integration: "tests/integration",
  e2e: "tests/e2e",
  contract: "tests/contract",
  security: "tests/security",
  property: "tests/property",
  fuzz: "tests/fuzz",
} as const;

type Family = keyof typeof FAMILY_DIRS;

function parseFamily(): Family | "all" {
  const typeArg = process.argv.find((arg) => arg.startsWith("--type="));
  if (!typeArg) {
    return "all";
  }
  const family = typeArg.slice("--type=".length) as Family;
  return family in FAMILY_DIRS ? family : "all";
}

function readPlaceholderFiles(targetFamily: Family | "all"): Record<string, string[]> {
  const families = targetFamily === "all" ? Object.keys(FAMILY_DIRS) as Family[] : [targetFamily];
  const payload: Record<string, string[]> = {};

  for (const family of families) {
    payload[family] = readdirSync(resolve(FAMILY_DIRS[family])).sort();
  }

  return payload;
}

const family = parseFamily();
console.error(
  JSON.stringify(
    {
      scope: "infra/cloudflare/scripts/test-runner.ts",
      status: "manual-required",
      requestedFamily: family,
      placeholderFiles: readPlaceholderFiles(family),
      blocker: "test-runner-scaffold-only",
      nextStep: "Replace placeholder tests and blocker runner with real suite execution plus proof output.",
    },
    null,
    2,
  ),
);

failWithBlocker(
  "infra/cloudflare/scripts/test-runner.ts",
  "test-runner-scaffold-only",
  "Wire real unit/integration/e2e/contract/security/property/fuzz execution before claiming Cloudflare proof.",
);
