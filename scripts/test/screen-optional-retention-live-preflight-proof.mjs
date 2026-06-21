import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(__dirname, '..', '..');
const proofDir = join(repoRoot, 'output', 'screen-plan-proof', '27-28-optional-retention-live-preflight');
const proofPath = join(proofDir, 'proof-summary.json');

run('npm', ['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
run('npm', ['run', 'build', '--workspace', '@ocentra-parent/screen-domain']);

const optionalVisibilityModeModule = await import(
  pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', 'screen-optional-visibility-mode.js')).href
);
const optionalVisibilityModeValuesModule = await import(
  pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', 'screen-optional-visibility-mode-values.js')).href
);

const screenEvidence = {
  ScreenOptionalVisibilityModeSchemaVersion: optionalVisibilityModeValuesModule.ScreenOptionalVisibilityModeSchemaVersion,
  ScreenRawScreenshotRetentionOptInSettingSchema:
    optionalVisibilityModeModule.ScreenRawScreenshotRetentionOptInSettingSchema,
  ScreenLiveViewOptInSettingSchema: optionalVisibilityModeModule.ScreenLiveViewOptInSettingSchema,
};

const generatedAt = new Date().toISOString();
const disabledRetention = retentionSetting('disabled', {});
const localRetention = retentionSetting('localShortTtl', {
  settingId: 'proof-screen-retention-local-ttl',
  explicitParentApproval: true,
  approvalRef: 'proof-screen-retention-approval-local',
  disclosureState: 'requiredShown',
  auditRef: 'proof-screen-retention-audit-local',
  ttlSeconds: 300,
  custodyState: 'child-device-temp-queue',
  sourceLabel: 'rawScreenshotRetention',
  retentionBehavior: 'deleteAfterTtl',
  deleteAfterTtl: true,
  deleteProofRequired: true,
  reason: 'parent approved short TTL local screenshot retention',
});
const exportRetention = retentionSetting('parentOwnedExport', {
  settingId: 'proof-screen-retention-parent-export',
  explicitParentApproval: true,
  approvalRef: 'proof-screen-retention-approval-export',
  disclosureState: 'requiredShown',
  auditRef: 'proof-screen-retention-audit-export',
  ttlSeconds: 3600,
  custodyState: 'parent-owned-export',
  exportRef: 'proof-screen-retention-parent-export-ref',
  sourceLabel: 'rawScreenshotRetention',
  retentionBehavior: 'parentOwnedExportDeleteOnRevoke',
  deleteAfterTtl: true,
  deleteProofRequired: true,
  reason: 'parent approved parent-owned screenshot export with revoke/delete behavior',
});
const disabledLiveView = liveViewSetting('disabled', {});
const lanLiveView = liveViewSetting('lanOnlyView', {
  settingId: 'proof-screen-live-view-lan',
  transportMode: 'lanMutualAuth',
  explicitParentApproval: true,
  approvalRef: 'proof-screen-live-view-approval-lan',
  disclosureState: 'requiredShown',
  viewerAuditRef: 'proof-screen-live-view-audit-lan',
  platformProofState: 'operatorVerified',
  platformProofRef: 'proof-screen-live-view-platform-lan',
  custodyState: 'live-lan-child-agent',
  sourceLabel: 'liveView',
  reason: 'parent approved LAN view-only live screen after platform proof',
});
const relayLiveView = liveViewSetting('relayBackedView', {
  settingId: 'proof-screen-live-view-relay',
  transportMode: 'relayEndToEndEncrypted',
  explicitParentApproval: true,
  approvalRef: 'proof-screen-live-view-approval-relay',
  disclosureState: 'requiredShown',
  viewerAuditRef: 'proof-screen-live-view-audit-relay',
  platformProofState: 'operatorVerified',
  platformProofRef: 'proof-screen-live-view-platform-relay',
  custodyState: 'ocentra-hosted-non-activity',
  sourceLabel: 'relay',
  reason: 'parent approved relay-backed view-only live screen after platform proof',
});

const accepted = [
  screenEvidence.ScreenRawScreenshotRetentionOptInSettingSchema.parse(disabledRetention),
  screenEvidence.ScreenRawScreenshotRetentionOptInSettingSchema.parse(localRetention),
  screenEvidence.ScreenRawScreenshotRetentionOptInSettingSchema.parse(exportRetention),
  screenEvidence.ScreenLiveViewOptInSettingSchema.parse(disabledLiveView),
  screenEvidence.ScreenLiveViewOptInSettingSchema.parse(lanLiveView),
  screenEvidence.ScreenLiveViewOptInSettingSchema.parse(relayLiveView),
];

const negativeChecks = [
  rejects('silent raw retention without parent approval', () =>
    screenEvidence.ScreenRawScreenshotRetentionOptInSettingSchema.safeParse({
      ...localRetention,
      explicitParentApproval: false,
    })
  ),
  rejects('raw retention without TTL', () =>
    screenEvidence.ScreenRawScreenshotRetentionOptInSettingSchema.safeParse({
      ...localRetention,
      ttlSeconds: null,
    })
  ),
  rejects('raw screenshot remote upload', () =>
    screenEvidence.ScreenRawScreenshotRetentionOptInSettingSchema.safeParse({
      ...localRetention,
      rawScreenshotRemoteUploadEnabled: true,
    })
  ),
  rejects('live view without platform proof', () =>
    screenEvidence.ScreenLiveViewOptInSettingSchema.safeParse({
      ...lanLiveView,
      platformProofState: 'missing',
    })
  ),
  rejects('live view with cached raw frames', () =>
    screenEvidence.ScreenLiveViewOptInSettingSchema.safeParse({
      ...lanLiveView,
      cacheRawFrames: true,
    })
  ),
  rejects('live view with remote input control', () =>
    screenEvidence.ScreenLiveViewOptInSettingSchema.safeParse({
      ...lanLiveView,
      remoteInputControlAllowed: true,
    })
  ),
];

if (negativeChecks.some((check) => !check.rejected)) {
  throw new Error('Expected optional retention/live-view negative rows to reject');
}

mkdirSync(proofDir, { recursive: true });
writeFileSync(
  proofPath,
  `${JSON.stringify(
    {
      schemaVersion: 1,
      generatedAt,
      claim:
        'Raw screenshot retention and live view are separate explicit opt-in modes with audit, custody, TTL/no-retention, deletion, and platform-proof gates.',
      acceptedModes: accepted.map((mode) => modeSummary(mode)),
      negativeChecks,
      nonClaims: [
        'No raw screenshot retention runtime is implemented or enabled by default.',
        'No live screen transport, relay, cache, platform permission prompt, or platform adapter is implemented by this proof.',
        'No policy action, enforcement action, remote input control, or production privacy/legal approval is claimed.',
      ],
    },
    null,
    2
  )}\n`
);

console.log(`screen-optional-retention-live-preflight-proof-ok:${proofPath}`);

function retentionSetting(mode, overrides) {
  return {
    schemaVersion: screenEvidence.ScreenOptionalVisibilityModeSchemaVersion,
    settingId: `proof-screen-retention-${mode}`,
    parentSettingRef: `proof-screen-retention-parent-${mode}`,
    settingVersion: 1,
    changedAt: generatedAt,
    mode,
    explicitParentApproval: false,
    approvalRef: null,
    disclosureState: 'notRequired',
    auditRef: null,
    ttlSeconds: null,
    custodyState: 'unavailable',
    exportRef: null,
    sourceLabel: 'unavailable',
    retentionBehavior: 'noRawRetention',
    deleteAfterTtl: false,
    deleteOnParentDisable: true,
    deleteProofRequired: false,
    rawScreenshotRemoteUploadEnabled: false,
    reason: 'raw screenshots are not retained by default',
    ...overrides,
  };
}

function liveViewSetting(liveViewMode, overrides) {
  return {
    schemaVersion: screenEvidence.ScreenOptionalVisibilityModeSchemaVersion,
    settingId: `proof-screen-live-view-${liveViewMode}`,
    parentSettingRef: `proof-screen-live-view-parent-${liveViewMode}`,
    settingVersion: 1,
    changedAt: generatedAt,
    liveViewMode,
    transportMode: 'none',
    explicitParentApproval: false,
    approvalRef: null,
    disclosureState: 'notRequired',
    viewerAuditRef: null,
    platformProofState: 'notRequired',
    platformProofRef: null,
    custodyState: 'unavailable',
    sourceLabel: 'unavailable',
    frameRetentionBehavior: 'noFrameRetention',
    cacheRawFrames: false,
    sessionRecordingAllowed: false,
    remoteInputControlAllowed: false,
    stopOrRevokeAuditRequired: true,
    reason: 'live view is disabled by default',
    ...overrides,
  };
}

function rejects(name, parseAttempt) {
  return { name, rejected: !parseAttempt().success };
}

function modeSummary(mode) {
  if ('mode' in mode) {
    return {
      settingId: mode.settingId,
      mode: mode.mode,
      custodyState: mode.custodyState,
      ttlSeconds: mode.ttlSeconds,
      exportRef: mode.exportRef,
      deleteProofRequired: mode.deleteProofRequired,
      rawScreenshotRemoteUploadEnabled: mode.rawScreenshotRemoteUploadEnabled,
    };
  }

  return {
    settingId: mode.settingId,
    liveViewMode: mode.liveViewMode,
    transportMode: mode.transportMode,
    custodyState: mode.custodyState,
    sourceLabel: mode.sourceLabel,
    platformProofState: mode.platformProofState,
    cacheRawFrames: mode.cacheRawFrames,
    remoteInputControlAllowed: mode.remoteInputControlAllowed,
  };
}

function run(command, args) {
  const runner = process.platform === 'win32' ? 'cmd' : command;
  const runnerArgs = process.platform === 'win32' ? ['/c', command, ...args] : args;
  execFileSync(runner, runnerArgs, { cwd: repoRoot, stdio: 'inherit' });
}
