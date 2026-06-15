import { describe, expect, it } from 'vitest';
import {
  ScreenDetectorPromptDefinitionSchema,
  ScreenDetectorPromptOutputSchema,
  ScreenDetectorPromptPackSchema,
  ScreenDetectorPromptPackSchemaVersion,
  ScreenDetectorRequiredIds,
} from '../../src/screen-evidence';

describe('screen detector prompt pack contracts', () => {
  it('accepts an active detector pack with one privacy-safe prompt per required detector', () => {
    const parsed = ScreenDetectorPromptPackSchema.safeParse(promptPack());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.detectors.map((detector) => detector.detectorId)).toEqual(ScreenDetectorRequiredIds);
      expect(parsed.data.degradedStates).toEqual([]);
    }
  });

  it('rejects duplicate or incomplete active detector packs', () => {
    const duplicate = ScreenDetectorPromptPackSchema.safeParse({
      ...promptPack(),
      detectors: [...promptPack().detectors, promptDefinition('socialVideo')],
    });
    const missing = ScreenDetectorPromptPackSchema.safeParse({
      ...promptPack(),
      detectors: promptPack().detectors.filter((detector) => detector.detectorId !== 'signupIdentity'),
    });

    expect(duplicate.success).toBe(false);
    expect(missing.success).toBe(false);
  });

  it('rejects prompt definitions that allow open-ended descriptions or private output fields', () => {
    const unsafePrompt = ScreenDetectorPromptDefinitionSchema.safeParse({
      ...promptDefinition('chatMessaging'),
      openEndedDescriptionAllowed: true,
      privateMessageTextAllowed: true,
      forbiddenOutputFields: ['rawScreenshotRef'],
    });

    expect(unsafePrompt.success).toBe(false);
  });

  it('accepts schema-valid detector output without raw/private fields or policy authority', () => {
    const parsed = ScreenDetectorPromptOutputSchema.safeParse(detectorOutput());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.detectorId).toBe('browserGame');
      expect(parsed.data.finalPolicyActionClaimed).toBe(false);
      expect(parsed.data.rawScreenshotRefIncluded).toBe(false);
    }
  });

  it('rejects unsafe output and low-confidence output without uncertainty', () => {
    const unsafe = ScreenDetectorPromptOutputSchema.safeParse({
      ...detectorOutput(),
      credentialTextIncluded: true,
      finalPolicyActionClaimed: true,
    });
    const uncertainWithoutReason = ScreenDetectorPromptOutputSchema.safeParse({
      ...detectorOutput(),
      confidence: 0.3,
      uncertaintyReasons: [],
    });

    expect(unsafe.success).toBe(false);
    expect(uncertainWithoutReason.success).toBe(false);
  });
});

function promptPack() {
  return {
    schemaVersion: ScreenDetectorPromptPackSchemaVersion,
    promptPackId: 'screen-detector-prompt-pack-v1',
    promptPackVersion: 'screen-detector-prompt-pack-2026-06-05',
    publishedAt: '2026-06-05T03:02:00.000Z',
    status: 'active',
    detectors: ScreenDetectorRequiredIds.map((detectorId) => promptDefinition(detectorId)),
    degradedStates: [],
    auditEvidenceIds: ['screen-detector-prompt-pack-audit'],
  };
}

function promptDefinition(detectorId: (typeof ScreenDetectorRequiredIds)[number]) {
  return {
    detectorId,
    promptPackId: 'screen-detector-prompt-pack-v1',
    promptPackVersion: 'screen-detector-prompt-pack-2026-06-05',
    promptHashRef: `screen-detector-prompt-hash-${detectorId}`,
    targetCategories: targetCategories(detectorId),
    targetRiskSignals: ['unknown'],
    allowedInputFields: ['sourceEvidenceRefs', 'ocrSnippets', 'visibleCategoryCandidates', 'safeImageCropRef'],
    outputFields: [
      'detectorId',
      'categoryCandidates',
      'riskSignals',
      'confidence',
      'uncertaintyReasons',
      'evidenceRefs',
      'redactionNotes',
      'childSafeSummary',
    ],
    forbiddenOutputFields: [
      'privateMessageText',
      'personName',
      'credentialText',
      'fullOcrText',
      'rawScreenshotRef',
      'rawPromptText',
      'accountIdentifier',
      'addressOrPhone',
    ],
    rawPromptTextIncluded: false,
    openEndedDescriptionAllowed: false,
    fullOcrTextAllowed: false,
    privateMessageTextAllowed: false,
    personalNamesAllowed: false,
    credentialTextAllowed: false,
    rawScreenshotRefAllowed: false,
    childSafetyOnly: true,
  };
}

function detectorOutput() {
  return {
    schemaVersion: ScreenDetectorPromptPackSchemaVersion,
    detectorId: 'browserGame',
    promptPackVersion: 'screen-detector-prompt-pack-2026-06-05',
    analyzedAt: '2026-06-05T03:03:00.000Z',
    sourceEvidenceIds: ['screen-detector-output-source'],
    primaryCategory: 'game',
    categoryCandidates: [{ category: 'game', confidence: 0.86, evidenceRefs: [evidenceRef()] }],
    riskSignals: [{ signal: 'unknown', confidence: 0.52, evidenceRefs: [evidenceRef()] }],
    ocrSnippets: [{ text: 'Start game', confidence: 0.73, evidenceRefs: [evidenceRef()] }],
    confidence: 0.86,
    uncertaintyReasons: [],
    redactionNotes: ['credentialLikeTextRedacted'],
    childSafeSummary: 'Visible screen signals match a browser game route.',
    privateMessageTextIncluded: false,
    personalNamesIncluded: false,
    credentialTextIncluded: false,
    fullOcrTextIncluded: false,
    rawScreenshotRefIncluded: false,
    finalPolicyActionClaimed: false,
    enforcementActionClaimed: false,
  };
}

function evidenceRef() {
  return {
    evidenceId: 'screen-detector-evidence-ref',
    kind: 'journal-entry',
    digest: 'screen-detector-evidence-digest',
    uri: null,
  };
}

function targetCategories(detectorId: (typeof ScreenDetectorRequiredIds)[number]) {
  const categories = {
    socialVideo: ['video'],
    chatMessaging: ['chat'],
    browserGame: ['game'],
    schoolProductivity: ['school', 'productivity'],
    bypassTool: ['bypassTool'],
    adultContent: ['adultContent'],
    violenceSafety: ['violence'],
    shoppingPayment: ['shopping'],
    signupIdentity: ['unknown'],
  } as const;
  return categories[detectorId];
}
