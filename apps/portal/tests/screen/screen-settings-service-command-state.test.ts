import { describe, expect, it } from 'vitest';
import {
  ParentScreenSettingsCommandRuntime,
  ParentScreenSettingsUpdateKind,
  ParentUiActionKind,
  ParentUiActionPayloadField,
} from '../../generated/parent-ui-bridge';
import {
  parentScreenEvidenceSettingsWritableUiProof as screenEvidenceSettingsWritableUiProof,
  type ParentScreenEvidenceSettingsUiProof,
} from '../../generated/parent-ui-screen-bridge';
import {
  createScreenSettingsGetCommandDraft,
  createScreenSettingsReplaceCommandDraft,
  decodeScreenSettingsServiceResponseSnapshot,
  matchingScreenSettingsServiceResponse,
  screenSettingsBaseVersionForReplace,
  screenSettingsServiceStatusText,
} from '../../src/screen-settings-service-command-state';

describe('screen settings service command state', () => {
  it('builds generated get command payloads for portal actions', verifyGeneratedGetCommandPayload);
  it('builds replace command payloads that preserve the strict screen setting contract', verifyReplaceCommandPayload);
  it('selects matching snapshot responses and derives visible status text', verifyMatchingSnapshotStatusText);
  it('decodes route snapshot responses and filters them by the pending request id', verifySnapshotResponseFiltering);
});

function verifyGeneratedGetCommandPayload() {
  const getDraft = createScreenSettingsGetCommandDraft(3);

  expect(getDraft.action).toBe(ParentUiActionKind.ScreenSettingsGetRequested);
  expect(getDraft.requestId).toBe(`${ParentScreenSettingsCommandRuntime.RequestIdPrefix}3`);
  expect(getDraft.payload[ParentUiActionPayloadField.ScreenSettingsUpdateKind]).toBe(
    ParentScreenSettingsUpdateKind.Get
  );
  expect(JSON.parse(String(getDraft.payload[ParentUiActionPayloadField.ScreenSettingsRequest]))).toEqual({
    schemaVersion: ParentScreenSettingsCommandRuntime.SchemaVersion,
    requestId: `${ParentScreenSettingsCommandRuntime.RequestIdPrefix}3`,
    kind: ParentScreenSettingsUpdateKind.Get,
  });
}

function verifyReplaceCommandPayload() {
  const proof = screenEvidenceSettingsWritableUiProof();
  const setting = requireStrictSetting(proof);
  const replaceDraft = createScreenSettingsReplaceCommandDraft({
    baseSettingVersion: setting.settingVersion,
    sequence: 4,
    setting,
  });

  expect(replaceDraft.action).toBe(ParentUiActionKind.ScreenSettingsReplaceRequested);
  expect(replaceDraft.requestId).toBe(`${ParentScreenSettingsCommandRuntime.RequestIdPrefix}4`);
  expect(JSON.parse(String(replaceDraft.payload[ParentUiActionPayloadField.ScreenSettingsRequest]))).toEqual({
    schemaVersion: ParentScreenSettingsCommandRuntime.SchemaVersion,
    requestId: `${ParentScreenSettingsCommandRuntime.RequestIdPrefix}4`,
    kind: ParentScreenSettingsUpdateKind.Replace,
    baseSettingVersion: 3,
    setting,
  });
}

function verifyMatchingSnapshotStatusText() {
  const proof = screenEvidenceSettingsWritableUiProof();
  const setting = requireStrictSetting(proof);
  const response = matchingScreenSettingsServiceResponse(
    decodeScreenSettingsServiceResponseSnapshot({
      schemaVersion: ParentScreenSettingsCommandRuntime.SchemaVersion,
      requestId: `${ParentScreenSettingsCommandRuntime.RequestIdPrefix}9`,
      kind: ParentScreenSettingsUpdateKind.Replace,
      status: 'accepted',
      setting,
      auditEventId: 'screen-settings-audit-screen-settings-request-9',
      rejectionReason: null,
      message: 'Screen settings update accepted.',
    }),
    `${ParentScreenSettingsCommandRuntime.RequestIdPrefix}9`
  );

  expect(response).toEqual({
    schemaVersion: ParentScreenSettingsCommandRuntime.SchemaVersion,
    requestId: `${ParentScreenSettingsCommandRuntime.RequestIdPrefix}9`,
    kind: ParentScreenSettingsUpdateKind.Replace,
    status: 'accepted',
    setting,
    auditEventId: 'screen-settings-audit-screen-settings-request-9',
    rejectionReason: null,
    message: 'Screen settings update accepted.',
  });
  expect(
    screenSettingsServiceStatusText({
      commandEnabled: true,
      pendingRequestId: `${ParentScreenSettingsCommandRuntime.RequestIdPrefix}9`,
      proof,
      response,
    })
  ).toBe(proof.serviceAcceptedStatus);
  expect(screenSettingsBaseVersionForReplace(response)).toBe(3);
}

function verifySnapshotResponseFiltering() {
  const proof = screenEvidenceSettingsWritableUiProof();
  const setting = requireStrictSetting(proof);
  const snapshotResponse = decodeScreenSettingsServiceResponseSnapshot({
    schemaVersion: ParentScreenSettingsCommandRuntime.SchemaVersion,
    requestId: `${ParentScreenSettingsCommandRuntime.RequestIdPrefix}12`,
    kind: ParentScreenSettingsUpdateKind.Replace,
    status: 'accepted',
    setting,
    auditEventId: 'screen-settings-audit-screen-settings-request-12',
    rejectionReason: null,
    message: 'Screen settings update accepted.',
  });

  expect(
    matchingScreenSettingsServiceResponse(snapshotResponse, `${ParentScreenSettingsCommandRuntime.RequestIdPrefix}12`)
  ).toEqual(snapshotResponse);
  expect(
    matchingScreenSettingsServiceResponse(snapshotResponse, `${ParentScreenSettingsCommandRuntime.RequestIdPrefix}99`)
  ).toBeNull();
}

function requireStrictSetting(proof: ParentScreenEvidenceSettingsUiProof) {
  const setting = proof.intents[2]?.setting;
  if (setting === undefined) {
    throw new Error('missing strict screen setting');
  }
  return setting;
}
