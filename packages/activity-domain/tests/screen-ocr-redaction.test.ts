import { describe, expect, it } from 'vitest';
import {
  ScreenOcrRedactionPolicySchema,
  ScreenOcrRedactionProofSchema,
  ScreenOcrRedactionSchemaVersion,
  redactScreenOcrText,
} from '../src/screen-ocr-redaction';
import { ScreenAnalysisParentSettingSchema } from '../src/screen-evidence-settings';

const EvidenceRef = {
  evidenceId: 'screen-ocr-redaction-evidence',
  kind: 'journal-entry',
  digest: 'sha256:screen-ocr-redaction-image',
  uri: null,
} as const;

const RedactionPolicy = {
  schemaVersion: ScreenOcrRedactionSchemaVersion,
  policyId: 'screen-ocr-redaction-policy',
  updatedAt: '2026-06-06T22:47:00.000Z',
  ocrTextEnabled: true,
  snippetLimit: 2,
  redactionMode: 'localSensitiveText',
  textRetentionMode: 'redactedSnippets',
  credentialSuppressionEnabled: true,
  piiRedactionEnabled: true,
  parentControlled: true,
  rawTextRetentionAllowed: false,
} as const;

const CandidateLines = [
  {
    text: 'Homework portal for jane@example.com',
    confidence: 0.91,
    evidenceRefs: [EvidenceRef],
  },
  {
    text: 'password reset code 123456',
    confidence: 0.88,
    evidenceRefs: [EvidenceRef],
  },
  {
    text: 'Call parent at 555-010-1234',
    confidence: 0.82,
    evidenceRefs: [EvidenceRef],
  },
] as const;

describe('screen OCR redaction contracts', () => {
  specifyRedactedSnippetContracts();
  specifyDisabledOcrTextContracts();
  specifyUnsafePolicyRejections();
  specifyRedactionProofBoundary();
  specifyParentSettingSelection();
});

function specifyRedactedSnippetContracts() {
  it('keeps OCR snippets bounded, redacts PII-like text, and suppresses credential-like text', () => {
    const policy = ScreenOcrRedactionPolicySchema.parse(RedactionPolicy);
    const result = redactScreenOcrText({
      policy,
      processedAt: '2026-06-06T22:48:00.000Z',
      lines: CandidateLines,
    });

    expect(result.snippets).toHaveLength(2);
    expect(result.snippets[0]?.text).toBe('Homework portal for [redacted-email]');
    expect(result.snippets[1]?.text).toBe('Call parent at [redacted-phone]');
    expect(result.suppressed).toHaveLength(1);
    expect(result.suppressed[0]?.sensitiveKind).toBe('credentialLikeText');
    expect(result.redactionNotes).toEqual(['piiLikeTextRedacted', 'credentialLikeTextRedacted']);
    expect(result.rawTextRetained).toBe(false);
    expect(result.remoteAiUsed).toBe(false);
  });
}

function specifyDisabledOcrTextContracts() {
  it('returns no snippets and a visible note when OCR text is disabled by parent setting', () => {
    const disabledPolicy = ScreenOcrRedactionPolicySchema.parse({
      ...RedactionPolicy,
      ocrTextEnabled: false,
      snippetLimit: 0,
      redactionMode: 'disabled',
      textRetentionMode: 'disabled',
      piiRedactionEnabled: false,
    });
    const result = redactScreenOcrText({
      policy: disabledPolicy,
      processedAt: '2026-06-06T22:49:00.000Z',
      lines: CandidateLines,
    });

    expect(result.snippets).toHaveLength(0);
    expect(result.redactionNotes).toEqual(['ocrDisabled']);
    expect(result.rawTextRetained).toBe(false);
  });
}

function specifyUnsafePolicyRejections() {
  it('rejects unbounded retention, non-parent-controlled settings, and disabled OCR with retained snippets', () => {
    const unbounded = ScreenOcrRedactionPolicySchema.safeParse({
      ...RedactionPolicy,
      snippetLimit: 6,
    });
    const notParentControlled = ScreenOcrRedactionPolicySchema.safeParse({
      ...RedactionPolicy,
      parentControlled: false,
    });
    const disabledWithRetention = ScreenOcrRedactionPolicySchema.safeParse({
      ...RedactionPolicy,
      ocrTextEnabled: false,
      snippetLimit: 2,
      textRetentionMode: 'redactedSnippets',
    });

    expect(unbounded.success).toBe(false);
    expect(notParentControlled.success).toBe(false);
    expect(disabledWithRetention.success).toBe(false);
  });
}

function specifyRedactionProofBoundary() {
  it('proves the redaction contract without raw text retention, raw image retention, remote AI, or runtime claims', () => {
    const policy = ScreenOcrRedactionPolicySchema.parse(RedactionPolicy);
    const result = redactScreenOcrText({
      policy,
      processedAt: '2026-06-06T22:50:00.000Z',
      lines: CandidateLines,
    });
    const proof = ScreenOcrRedactionProofSchema.parse({
      schemaVersion: ScreenOcrRedactionSchemaVersion,
      proofId: 'screen-ocr-redaction-proof',
      proofTier: 'P2_CONTRACT_SCREEN_OCR_REDACTION',
      policy,
      result,
      credentialSuppressed: true,
      piiRedacted: true,
      disabledStateProved: true,
      localOnly: true,
      rawTextRetained: false,
      rawImageRetained: false,
      remoteAiUsed: false,
      portalRuntimeClaimed: false,
      servicePersistenceClaimed: false,
    });
    const remoteAi = ScreenOcrRedactionProofSchema.safeParse({
      ...proof,
      remoteAiUsed: true,
    });

    expect(proof.result.redactionNotes).toEqual(['piiLikeTextRedacted', 'credentialLikeTextRedacted']);
    expect(remoteAi.success).toBe(false);
  });
}

function specifyParentSettingSelection() {
  it('keeps OCR text retention and redaction as explicit parent-selected settings', () => {
    const selectedSetting = ScreenAnalysisParentSettingSchema.parse({
      schemaVersion: 1,
      screenAnalysisEnabled: true,
      analysisMode: 'policyDryRun',
      cadenceCaptureEnabled: true,
      cadenceSeconds: 60,
      strictModeEnabled: true,
      triggerCaptureEnabled: true,
      enabledTriggers: ['timedCadence'],
      allowedCaptureScope: 'activeWindow',
      ocrTextEnabled: true,
      ocrTextSnippetLimit: 2,
      redactionMode: 'localSensitiveText',
      ocrTextRetentionMode: 'redactedSnippets',
      credentialSuppressionEnabled: true,
      piiRedactionEnabled: true,
      temporaryImageTtlSeconds: 60,
      maxRetryCount: 1,
      deleteAfterSuccess: true,
      deleteAfterExpiry: true,
      retainRawImage: false,
      policyUseEnabled: true,
      changedByParentRef: 'parent-setting-screen-1',
      changedAt: '2026-06-06T22:51:00.000Z',
      settingVersion: 3,
      reason: 'parent selected local sensitive OCR snippet redaction',
    });
    const disabledSetting = ScreenAnalysisParentSettingSchema.parse({
      ...selectedSetting,
      ocrTextEnabled: false,
      ocrTextSnippetLimit: 0,
      redactionMode: 'disabled',
      ocrTextRetentionMode: 'disabled',
      piiRedactionEnabled: false,
      settingVersion: 4,
    });
    const unsafeDisabled = ScreenAnalysisParentSettingSchema.safeParse({
      ...selectedSetting,
      ocrTextEnabled: false,
    });

    expect(selectedSetting.ocrTextRetentionMode).toBe('redactedSnippets');
    expect(selectedSetting.credentialSuppressionEnabled).toBe(true);
    expect(selectedSetting.piiRedactionEnabled).toBe(true);
    expect(disabledSetting.ocrTextSnippetLimit).toBe(0);
    expect(unsafeDisabled.success).toBe(false);
  });
}
