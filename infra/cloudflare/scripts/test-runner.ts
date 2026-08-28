#!/usr/bin/env node

import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

type TestFamily = 'unit' | 'integration' | 'e2e' | 'contract' | 'security' | 'property' | 'fuzz';

interface FamilyConfig {
  readonly directory: string;
  readonly files: ReadonlyArray<string>;
  readonly assertionIds: ReadonlyArray<string>;
  readonly proofId: string;
}

const FAMILY_ORDER: ReadonlyArray<TestFamily> = [
  'unit',
  'integration',
  'e2e',
  'contract',
  'security',
  'property',
  'fuzz',
];

const FAMILY_CONFIG: Record<TestFamily, FamilyConfig> = {
  unit: {
    directory: 'tests/unit',
    files: [
      'tests/unit/auth-boundary.test.ts',
      'tests/unit/account-identity-store.test.ts',
      'tests/unit/billing-binding-read-model.test.ts',
      'tests/unit/route-manifest.test.ts',
      'tests/unit/env-bindings.test.ts',
      'tests/unit/redaction.test.ts',
      'tests/unit/request-limits.test.ts',
      'tests/unit/kill-switch.test.ts',
    ],
    assertionIds: [
      'UT-AUTH-01',
      'UT-AUTH-02',
      'UT-AUTH-03',
      'UT-AUTH-04',
      'UT-AUTH-05',
      'UT-AUTH-06',
      'UT-AUTH-07',
      'UT-AUTH-08',
      'UT-AUTH-09',
      'UT-ACCOUNT-IDENTITY-01',
      'UT-ACCOUNT-IDENTITY-02',
      'UT-ACCOUNT-IDENTITY-03',
      'UT-ACCOUNT-IDENTITY-04',
      'UT-ROUTE-01',
      'UT-ROUTE-02',
      'UT-ROUTE-03',
      'UT-ROUTE-04',
      'UT-ROUTE-05',
      'UT-ROUTE-06',
      'UT-ROUTE-07',
      'UT-ROUTE-08',
      'UT-ROUTE-09',
      'UT-ROUTE-10',
      'UT-ENV-01',
      'UT-ENV-02',
      'UT-ENV-03',
      'UT-ENV-04',
      'UT-ENV-05',
      'UT-ENV-06',
      'UT-ENV-07',
      'UT-ENV-08',
      'UT-REDACT-01',
      'UT-REDACT-02',
      'UT-REDACT-03',
      'UT-REDACT-04',
      'UT-REDACT-05',
      'UT-REDACT-06',
      'UT-LIMIT-01',
      'UT-LIMIT-02',
      'UT-LIMIT-03',
      'UT-LIMIT-04',
      'UT-KILL-01',
      'UT-KILL-02',
      'UT-KILL-03',
      'UT-KILL-04',
      'UT-KILL-05',
    ],
    proofId: 'cloudflare-control.test-runner-unit',
  },
  integration: {
    directory: 'tests/integration',
    files: [
      'tests/integration/admin-auth-rejection.test.ts',
      'tests/integration/billing-status-auth.test.ts',
      'tests/integration/checkout-portal-hosted.test.ts',
      'tests/integration/local-dev-seeding-workflow.test.ts',
      'tests/integration/payment-routes-real.test.ts',
      'tests/integration/pricing-public.test.ts',
      'tests/integration/provider-webhooks.test.ts',
      'tests/integration/reconciliation-auth-boundary.test.ts',
      'tests/integration/webhook-signature-rejection.test.ts',
      'tests/integration/worker-health.test.ts',
      'tests/integration/worker-runtime-real.test.ts',
    ],
    assertionIds: [
      'IT-HEALTH-01',
      'IT-HEALTH-02',
      'IT-HEALTH-03',
      'IT-PRICE-01',
      'IT-PRICE-02',
      'IT-PRICE-03',
      'IT-STATUS-01',
      'IT-STATUS-02',
      'IT-STATUS-03',
      'IT-STATUS-04',
      'IT-WEBHOOK-01',
      'IT-WEBHOOK-02',
      'IT-WEBHOOK-03',
      'IT-WEBHOOK-04',
      'IT-ADMIN-01',
      'IT-ADMIN-02',
      'IT-ADMIN-03',
      'IT-ADMIN-04',
    ],
    proofId: 'cloudflare-control.test-runner-integration',
  },
  e2e: {
    directory: 'tests/e2e',
    files: ['tests/e2e/portal-to-worker-billing-status.test.ts'],
    assertionIds: ['E2E-PORTAL-01', 'E2E-PORTAL-02', 'E2E-PORTAL-03', 'E2E-PORTAL-04'],
    proofId: 'cloudflare-control.test-runner-e2e',
  },
  contract: {
    directory: 'tests/contract',
    files: ['tests/contract/billing-api-contract.test.ts'],
    assertionIds: ['CT-CONTRACT-01', 'CT-CONTRACT-02', 'CT-CONTRACT-03', 'CT-CONTRACT-04', 'CT-CONTRACT-05'],
    proofId: 'cloudflare-control.test-runner-contract',
  },
  security: {
    directory: 'tests/security',
    files: [
      'tests/security/no-provider-secrets-in-client.test.ts',
      'tests/security/cors-origin-rejection.test.ts',
      'tests/security/interactive-billing-boundary.test.ts',
      'tests/security/request-smuggling.test.ts',
      'tests/security/redaction.test.ts',
      'tests/security/payment-security-boundaries.test.ts',
    ],
    assertionIds: [
      'SEC-SECRETS-01',
      'SEC-SECRETS-02',
      'SEC-SECRETS-03',
      'SEC-SECRETS-04',
      'SEC-CORS-01',
      'SEC-CORS-02',
      'SEC-CORS-03',
      'SEC-SMUGGLE-01',
      'SEC-SMUGGLE-02',
      'SEC-SMUGGLE-03',
      'SEC-SMUGGLE-04',
      'SEC-REDACT-01',
      'SEC-REDACT-02',
      'SEC-REDACT-03',
    ],
    proofId: 'cloudflare-control.test-runner-security',
  },
  property: {
    directory: 'tests/property',
    files: ['tests/property/route-auth-state.property.test.ts', 'tests/property/billing-idempotency.property.test.ts'],
    assertionIds: [
      'PROP-ROUTE-01',
      'PROP-ROUTE-02',
      'PROP-ROUTE-03',
      'PROP-ROUTE-04',
      'PROP-ROUTE-05',
      'PROP-IDEMP-01',
      'PROP-IDEMP-02',
      'PROP-IDEMP-03',
      'PROP-IDEMP-04',
    ],
    proofId: 'cloudflare-control.property-fuzz-boundary',
  },
  fuzz: {
    directory: 'tests/fuzz',
    files: ['tests/fuzz/provider-webhook-payload.fuzz.test.ts'],
    assertionIds: ['FUZZ-WEBHOOK-01', 'FUZZ-WEBHOOK-02', 'FUZZ-WEBHOOK-03', 'FUZZ-WEBHOOK-04', 'FUZZ-WEBHOOK-05'],
    proofId: 'cloudflare-control.property-fuzz-boundary',
  },
};

const scriptDirectory = path.dirname(fileURLToPath(import.meta.url));
const moduleRoot = path.resolve(scriptDirectory, '..');

function parseFamilyArg(argv: ReadonlyArray<string>): TestFamily | 'all' {
  const typeArg = argv.find((arg) => arg.startsWith('--type='));
  if (typeArg === undefined) {
    return 'all';
  }

  const value = typeArg.slice('--type='.length);
  if (value === 'all') {
    return 'all';
  }

  if (FAMILY_ORDER.includes(value as TestFamily)) {
    return value as TestFamily;
  }

  console.error(`Unknown cloudflare test type: ${value}`);
  process.exit(1);
}

function collectDiscoveredTests(directory: string): string[] {
  const resolvedDirectory = path.resolve(moduleRoot, directory);
  if (!fs.existsSync(resolvedDirectory)) {
    return [];
  }

  const discovered: string[] = [];
  const stack = [resolvedDirectory];

  while (stack.length > 0) {
    const current = stack.pop();
    if (current === undefined) {
      continue;
    }

    const stat = fs.statSync(current);
    if (stat.isDirectory()) {
      for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
        stack.push(path.join(current, entry.name));
      }
      continue;
    }

    if (current.endsWith('.test.ts')) {
      discovered.push(path.relative(moduleRoot, current).replaceAll('\\', '/'));
    }
  }

  discovered.sort();
  return discovered;
}

function selectFamilies(selection: TestFamily | 'all'): TestFamily[] {
  return selection === 'all' ? [...FAMILY_ORDER] : [selection];
}

function validateSelection(families: ReadonlyArray<TestFamily>) {
  const missingFiles: string[] = [];
  const unexpectedFilesByFamily: Array<{ family: TestFamily; files: string[] }> = [];

  for (const family of families) {
    const config = FAMILY_CONFIG[family];
    for (const file of config.files) {
      if (!fs.existsSync(path.resolve(moduleRoot, file))) {
        missingFiles.push(file);
      }
    }

    const expected = new Set(config.files);
    const unexpected = collectDiscoveredTests(config.directory).filter((file) => !expected.has(file));
    if (unexpected.length > 0) {
      unexpectedFilesByFamily.push({ family, files: unexpected });
    }
  }

  if (missingFiles.length > 0) {
    console.error(`Missing required cloudflare test files:\n${missingFiles.map((file) => `- ${file}`).join('\n')}`);
    process.exit(1);
  }

  return unexpectedFilesByFamily;
}

function buildManifest(families: ReadonlyArray<TestFamily>) {
  return families.map((family) => ({
    family,
    proofId: FAMILY_CONFIG[family].proofId,
    files: FAMILY_CONFIG[family].files,
    assertionIds: FAMILY_CONFIG[family].assertionIds,
  }));
}

const selectedFamily = parseFamilyArg(process.argv);
const listOnly = process.argv.includes('--list');
const selectedFamilies = selectFamilies(selectedFamily);
const unexpectedFilesByFamily = validateSelection(selectedFamilies);
const manifest = buildManifest(selectedFamilies);
const selectedFiles = manifest.flatMap((entry) => entry.files);

if (listOnly) {
  console.log(
    JSON.stringify(
      {
        selectedFamily,
        selectedFamilies,
        families: manifest,
        unexpectedFilesByFamily,
      },
      null,
      2
    )
  );
  process.exit(0);
}

console.log(
  JSON.stringify(
    {
      selectedFamily,
      selectedFamilies,
      families: manifest,
      unexpectedFilesByFamily,
    },
    null,
    2
  )
);

const result = spawnSync(process.execPath, ['--import', 'tsx', '--test', ...selectedFiles], {
  cwd: moduleRoot,
  stdio: 'inherit',
});

process.exit(result.status ?? 1);
