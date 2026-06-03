import { writeFile } from 'node:fs/promises';
import { performance } from 'node:perf_hooks';
import { createParentPortalActivityUiIntent } from '../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent.ts';

const startedAt = performance.now();
const intent = createParentPortalActivityUiIntent({
  activityAppUseReadModel: adapterResult(readModelRows("app", 250)),
  activityGamesReadModel: adapterResult(readModelRows("game", 250)),
}, 5);
const observedMs = Number((performance.now() - startedAt).toFixed(3));
const metrics = Object.fromEntries(intent.appGameDashboard.metrics.map((metric) => [metric.label, metric.value]));
if (intent.appGameDashboard.rows.length !== 500) {
  throw new Error(`expected 500 app/game dashboard rows, got ${intent.appGameDashboard.rows.length}`);
}
if (metrics["App rows"] !== "250" || metrics["Game rows"] !== "250") {
  throw new Error(`unexpected app/game row metrics ${JSON.stringify(metrics)}`);
}
if (!intent.appGameDashboard.capabilityRows.some((row) => row.label === "manual-required")) {
  throw new Error("expected manual-required capability rows to stay visible");
}
await writeFile(new URL("./portal-scale-proof.json", import.meta.url), JSON.stringify({
  observedMs,
  totalRows: intent.appGameDashboard.rows.length,
  appRows: intent.appGameDashboard.appRows.length,
  gameRows: intent.appGameDashboard.gameRows.length,
  summary: intent.appGameDashboard.summary,
  metrics,
  capabilityLabels: intent.appGameDashboard.capabilityRows.map((row) => row.label),
}, null, 2));

function adapterResult(value) {
  return { ok: true, state: value.state, value };
}

function readModelRows(kind, count) {
  return {
    schemaVersion: 1,
    request: {
      schemaVersion: 1,
      scope: { scopeKind: "device", familyId: null, deviceId: "child-device-1" },
      requestedAt: "2026-06-03T12:42:00.000Z",
      rangeStart: "2026-06-03T00:00:00.000Z",
      rangeEnd: "2026-06-03T12:42:00.000Z",
    },
    state: kind === "game" ? "manual-required" : "ready",
    generatedAt: "2026-06-03T12:42:01.000Z",
    summary: `${kind} scale read model`,
    rows: Array.from({ length: count }, (_, index) => row(kind, index)),
  };
}

function row(kind, index) {
  const manual = index % 10 === 0;
  const foreground = index % 4 === 0;
  const running = index % 3 !== 0;
  const base = {
    rowId: `${kind}-row-${index}`,
    deviceId: `child-device-${index % 5}`,
    state: manual ? "manual-required" : "ready",
    productKind: kind === "game" ? "native-game" : "native-app",
    classificationState: manual ? "unknown-process" : kind === "game" ? "known-game" : "known-app",
    inventoryState: "installed",
    runtimeState: running ? "running" : "not-running",
    foregroundState: foreground ? "foreground" : "not-foreground",
    capabilityStatus: manual ? "manual-required" : "ready",
    lastObservedAt: "2026-06-03T12:42:00.000Z",
    totalMs: 60000 + index,
    inventoryRowCount: 1,
    runningRowCount: running ? 1 : 0,
    foregroundRowCount: foreground ? 1 : 0,
    dailyRollupCount: 1,
    evidence: [{ evidenceId: `${kind}-evidence-${index}`, sourceId: "sqlite", capturedAt: "2026-06-03T12:42:00.000Z" }],
  };
  if (kind === "game") {
    return { ...base, displayName: `Scale Game ${index}`, sessionCount: 1, launcherRowCount: index % 8 === 0 ? 1 : 0 };
  }
  return { ...base, appName: `Scale App ${index}`, launchCount: 1 };
}