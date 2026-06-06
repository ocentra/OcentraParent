import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const root = process.cwd();
const wp03Output = join(root, 'output', 'browser-plan-proof', '03-browser-inventory-model');
const wp04Output = join(root, 'output', 'browser-plan-proof', '04-windows-browser-inventory-adapter');
const wp14Output = join(root, 'output', 'browser-plan-proof', '14-portal-browser-status-surfaces');
const resultDirectory = join(root, 'test-results', 'browser-inventory-model-completion-proof');
const proofPath = join(resultDirectory, 'proof.json');
const manifestPath = join(wp03Output, '11-completion-proof-gate.md');

const requiredWp03Files = [
  '00-source-snapshot.md',
  '01-contract-proof.log',
  '02-rust-protocol-proof.log',
  '03-runtime-evidence.json',
  '04-journal-sqlite-proof.json',
  '05-policy-action-proof.json',
  '06-ui-snapshots/ui-not-applicable.md',
  '07-playwright-ui-proof.log',
  '08-security-negative-proof.log',
  '09-manual-platform-proof.md',
  '10-validation-commands.log',
];

await main();

async function main() {
  const wp03Validation = await readText(join(wp03Output, '10-validation-commands.log'));
  const wp14Validation = await readText(join(wp14Output, '10-validation-commands.log'));
  const portalInventoryJson = await readJson(
    join(wp14Output, '06-ui-snapshots', 'browser-route-inventory-status.json')
  );
  const windowsInventoryProof = await readJson(
    join(root, 'test-results', 'browser-windows-live-inventory-proof', 'proof.json')
  );
  const portalInventoryTest = await readText(
    join(root, 'apps', 'portal', 'tests', 'live-activity-browser-status.test.ts')
  );

  const proofFiles = [
    ...requiredWp03Files.map((file) => proofFile(wp03Output, file)),
    proofFile(wp04Output, '09-manual-platform-proof.md'),
    proofFile(wp04Output, '10-validation-commands.log'),
    proofFile(wp14Output, '06-ui-snapshots/browser-route-inventory-status.png'),
    proofFile(wp14Output, '06-ui-snapshots/browser-route-inventory-status.json'),
    proofFile(wp14Output, '10-validation-commands.log'),
  ];

  const checks = [
    checkProofFiles(proofFiles),
    checkLog(
      wp03Validation,
      'cmd /c npm run test --workspace @ocentra-parent/activity-domain -- browser-inventory.test.ts'
    ),
    checkLog(wp03Validation, 'cargo test -p ocentra-parent-agent-protocol browser_inventory --quiet'),
    checkLog(wp03Validation, 'cargo test -p ocentra-parent-agent-service browser_inventory_read_model --quiet'),
    checkLog(wp14Validation, 'Headless Playwright screenshot: http://127.0.0.1'),
    checkLog(wp14Validation, 'Browser inventory'),
    checkPortalInventoryArtifact(portalInventoryJson),
    checkWindowsInventoryProof(windowsInventoryProof),
    checkPortalParserTest(portalInventoryTest),
  ];
  const failures = checks.flatMap((check) => check.failures);
  const proof = {
    schemaVersion: 1,
    proofMode: 'browser-inventory-model-completion-proof',
    generatedAt: new Date().toISOString(),
    sourceWorkpack: 'docs/plans/browser-plan/workpacks/03-browser-inventory-model.md',
    evidenceWorkpacks: [
      'docs/plans/browser-plan/workpacks/04-windows-browser-inventory-adapter.md',
      'docs/plans/browser-plan/workpacks/14-portal-browser-status-surfaces.md',
    ],
    summary: {
      status: failures.length === 0 ? 'complete-with-no-claim-boundaries' : 'failed',
      proofFilesChecked: proofFiles.length,
      checksPassed: checks.filter((check) => check.failures.length === 0).length,
      failures: failures.length,
      productChecklistUpgradeClaimed: false,
    },
    checks,
    proofFiles,
    noClaimBoundaries: [
      'inventory-model-does-not-prove-exact-url',
      'inventory-model-does-not-prove-active-tab',
      'inventory-model-does-not-prove-blocking-or-enforcement',
      'non-windows-platform-adapters-remain-in-wp05',
      'appx-msix-and-lnk-binary-parsing-remain-in-wp04',
    ],
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Browser inventory model completion proof failed:\n${failures.join('\n')}`);
  }

  await mkdir(resultDirectory, { recursive: true });
  await mkdir(wp03Output, { recursive: true });
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(manifestPath, `${markdownFor(proof)}\n`);

  console.log('browser-inventory-model-completion-proof-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`manifest=${relativePath(manifestPath)}`);
  console.log(`checks=${proof.summary.checksPassed} proofFiles=${proof.summary.proofFilesChecked}`);
}

function checkProofFiles(files) {
  return {
    id: 'required-proof-files-exist',
    status: files.every((file) => file.exists) ? 'pass' : 'fail',
    failures: files.filter((file) => !file.exists).map((file) => `missing proof artifact: ${file.path}`),
  };
}

function checkLog(contents, needle) {
  return {
    id: `log-contains-${needle.replaceAll(' ', '-').slice(0, 48)}`,
    status: contents.includes(needle) ? 'pass' : 'fail',
    failures: contents.includes(needle) ? [] : [`expected validation log to contain: ${needle}`],
  };
}

function checkPortalInventoryArtifact(artifact) {
  const text = JSON.stringify(artifact);
  const requiredTokens = ['Browser inventory', 'Exact URL capability', 'Active tab proof'];
  return {
    id: 'portal-browser-route-inventory-artifact',
    status: requiredTokens.every((token) => text.includes(token)) ? 'pass' : 'fail',
    failures: requiredTokens
      .filter((token) => !text.includes(token))
      .map((token) => `portal inventory artifact missing token: ${token}`),
  };
}

function checkWindowsInventoryProof(proof) {
  const summary = proof.summary ?? {};
  const managementTierCounts = summary.managementTierCounts ?? {};
  const failures = [];
  if ((managementTierCounts.managed ?? 0) < 1) {
    failures.push('Windows live inventory proof has no managed rows');
  }
  if ((summary.totalRows ?? 0) < 3) {
    failures.push('Windows live inventory proof has fewer than three rows');
  }
  if (summary.productClaimed !== false) {
    failures.push('Windows live inventory proof unexpectedly claims product completion');
  }
  if (summary.exactUrlClaimedRows !== 0) {
    failures.push('Windows live inventory proof unexpectedly claims exact URL rows');
  }
  return {
    id: 'windows-live-inventory-proof-summary',
    status: failures.length === 0 ? 'pass' : 'fail',
    failures,
  };
}

function checkPortalParserTest(source) {
  const requiredTokens = [
    'parses browser inventory read-model events without upgrading unmanaged browser claims',
    "exactUrlCapability).toBe('not-claimed')",
    "activeTabCapability).toBe('not-claimed')",
    'publisherSignatureRef',
    'fileHashRef',
  ];
  return {
    id: 'portal-parser-test-keeps-no-claim-boundary',
    status: requiredTokens.every((token) => source.includes(token)) ? 'pass' : 'fail',
    failures: requiredTokens
      .filter((token) => !source.includes(token))
      .map((token) => `portal inventory parser test missing token: ${token}`),
  };
}

function proofFile(directory, file) {
  const absolutePath = join(directory, file);
  return {
    path: relativePath(absolutePath),
    exists: existsSync(absolutePath),
  };
}

async function readText(path) {
  return readFile(path, 'utf8');
}

async function readJson(path) {
  return JSON.parse(await readText(path));
}

function markdownFor(proof) {
  const checkRows = proof.checks
    .map((check) => `| ${check.id} | ${check.status} | ${check.failures.length} |`)
    .join('\n');
  const fileRows = proof.proofFiles.map((file) => `| ${file.path} | ${file.exists ? 'yes' : 'no'} |`).join('\n');

  return [
    '# WP03 Browser Inventory Model Completion Proof Gate',
    '',
    `Generated: ${proof.generatedAt}`,
    '',
    `Status: ${proof.summary.status}`,
    `Product checklist upgrade claimed: ${proof.summary.productChecklistUpgradeClaimed}`,
    '',
    'This gate closes the model row by verifying the WP03 contract/runtime proof pack, the WP04 live Windows inventory proof, and the WP14 portal inventory surface proof. It does not claim exact URL, known active tab, blocking, enforcement, or cross-platform adapter completion.',
    '',
    '## Checks',
    '',
    '| Check | Status | Failures |',
    '| --- | --- | --- |',
    checkRows,
    '',
    '## Proof Files',
    '',
    '| File | Exists |',
    '| --- | --- |',
    fileRows,
  ].join('\n');
}

function relativePath(path) {
  return relative(root, path).replaceAll('\\', '/');
}
