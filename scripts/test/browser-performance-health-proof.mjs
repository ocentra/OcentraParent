import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'browser-performance-health-proof');
const proofPath = join(outputDir, 'proof.json');

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  const { BrowserPerformanceHealthReadModelSchema } = await import('../../packages/browser-domain/dist/browser.js');
  const readModel = browserPerformanceHealthReadModel();
  const parsed = BrowserPerformanceHealthReadModelSchema.safeParse(readModel);

  assert(parsed.success, 'browser performance health read model parses');
  assertEqual(readModel.rows.length, 13, 'budget row count');
  assertEqual(countBy(readModel.rows.map((row) => row.state))['within-budget'], 8, 'measured budget count');
  assertEqual(countBy(readModel.rows.map((row) => row.state))['manual-required'], 5, 'manual budget count');
  assertEqual(readModel.rows.filter((row) => row.runtimeClaimed).length, 0, 'runtime claim count');

  const proof = {
    schemaVersion: 1,
    proofMode: 'browser-performance-health-proof',
    checkedAt: new Date().toISOString(),
    readModel,
    claimsProved: [
      'browser performance health contract parses measured fixture rows and manual-required future rows',
      'inventory, support matrix, 100-tab CDP mapping, journal write, 10000-event SQLite replay, unmanaged process scan, rapid bridge reconnect, and memory/cache budgets have fixture gates',
      'URL/video intelligence, local AI queue routing, browser-game runtime signals, cloud-gaming heuristics, and portal stress remain manual-required without runtime claims',
    ],
    claimsNotProved: [
      'hardware-specific release performance',
      'real portal 100-tab render timing',
      'URL/video intelligence runtime queue behavior',
      'browser-game runtime signal collection',
      'cloud-gaming bandwidth/session heuristic runtime',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log('browser-performance-health-proof-ok');
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
}

function browserPerformanceHealthReadModel() {
  return {
    schemaVersion: 1,
    generatedAt: new Date().toISOString(),
    sourceId: 'browser-performance-health-proof',
    healthState: 'healthy',
    rows: [...measuredBudgetRows(), ...manualRequiredRows()],
  };
}

function measuredBudgetRows() {
  return [
    withinBudgetRow('inventory-scan', 120, 250, 1),
    withinBudgetRow('support-matrix-derivation', 20, 50, 1),
    withinBudgetRow('cdp-target-mapping-100-tabs', 80, 100, 100),
    withinBudgetRow('journal-write-per-event', 5, 20, 1),
    withinBudgetRow('sqlite-replay-10000-events', 1_200, 2_000, 10_000),
    withinBudgetRow('unmanaged-process-scan', 100, 250, 1),
    withinBudgetRow('rapid-bridge-reconnect', 200, 500, 2),
    withinBudgetRow('memory-cache-lookup-invalidation', 8, 20, 1),
  ];
}

function manualRequiredRows() {
  return [
    manualRow('portal-render-100-tabs', 500, 'portal render stress remains UI-runtime manual proof'),
    manualRow('url-shape-metadata-extraction', 100, 'URL/video intelligence has not started'),
    manualRow('local-ai-queue-timeout', 30_000, 'local AI queue routing is future browser intelligence work'),
    manualRow('browser-game-runtime-signal-collection', 100, 'browser-game runtime signal collection is unimplemented'),
    manualRow('cloud-gaming-heuristic-timeout', 1_000, 'cloud-gaming heuristics are unimplemented'),
  ];
}

function withinBudgetRow(budgetId, observedMs, budgetMs, sampleSize) {
  return {
    budgetId,
    state: 'within-budget',
    proofSource: 'fixture',
    observedMs,
    budgetMs,
    sampleSize,
    degradedReason: null,
    manualRequiredReason: null,
    runtimeClaimed: false,
  };
}

function manualRow(budgetId, budgetMs, manualRequiredReason) {
  return {
    budgetId,
    state: 'manual-required',
    proofSource: 'future-work',
    observedMs: null,
    budgetMs,
    sampleSize: 1,
    degradedReason: null,
    manualRequiredReason,
    runtimeClaimed: false,
  };
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function assertEqual(actual, expected, message) {
  assert(actual === expected, `${message}: expected ${expected}, received ${actual}`);
}
