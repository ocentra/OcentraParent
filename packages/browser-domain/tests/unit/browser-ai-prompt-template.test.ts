import { describe, expect, it } from 'vitest';
import { BrowserAiPromptTemplateVersioningSchemaVersion } from '@ocentra-parent/schema-domain/browser-ai-prompt-template-schemas';
import {
  BrowserAiPromptTemplateRegistrySchema,
  BrowserAiPromptTemplateVersionRecordSchema,
  selectBrowserAiPromptTemplate,
} from '@ocentra-parent/schema-domain/browser-ai-prompt-template-schemas';

describe('browser AI prompt template versioning contract', () => {
  it('accepts active prompt template versions with audit refs and memory invalidation', acceptsActivePromptVersion);
  it('rejects raw prompt text and captured page content', rejectsRawPromptTextAndCapturedContent);
  it('rejects input-field changes without invalidating memory', rejectsInputFieldChangesWithoutMemoryInvalidation);
  it('selects one active prompt version for the requested task model and policy', selectsOneActivePromptVersion);
  it('rejects duplicate active prompt versions for the same task and model runtime', rejectsDuplicateActiveVersions);
  it(
    'returns manual-required when the active prompt does not support the selected model',
    returnsManualForUnsupportedModel
  );
  it('does not select deprecated prompt versions', doesNotSelectDeprecatedVersions);
});

function acceptsActivePromptVersion() {
  const parsed = BrowserAiPromptTemplateVersionRecordSchema.safeParse(activeVersionRecord());

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.status).toBe('active');
    expect(parsed.data.invalidatesMemory).toBe(true);
    expect(parsed.data.template.rawPromptTextIncluded).toBe(false);
    expect(parsed.data.auditEvidenceIds).toEqual(['prompt-template-audit-video-v2']);
  }
}

function rejectsRawPromptTextAndCapturedContent() {
  const parsed = BrowserAiPromptTemplateVersionRecordSchema.safeParse({
    ...activeVersionRecord(),
    template: {
      ...promptTemplate(),
      rawPromptTextIncluded: true,
      capturesRawPageBody: true,
      capturesTranscriptText: true,
    },
  });

  expect(parsed.success).toBe(false);
}

function rejectsInputFieldChangesWithoutMemoryInvalidation() {
  const parsed = BrowserAiPromptTemplateVersionRecordSchema.safeParse({
    ...activeVersionRecord(),
    changeReasons: ['model-change'],
    invalidatesMemory: false,
    inputFieldRefsChanged: true,
  });

  expect(parsed.success).toBe(false);
}

function selectsOneActivePromptVersion() {
  const selection = selectBrowserAiPromptTemplate(selectionRequest());

  expect(selection.selectionState).toBe('selected');
  expect(selection.selectedPromptTemplate?.promptTemplateVersion).toBe('browser-ai-video-safety-template-v2');
  expect(selection.degradedStates).toEqual([]);
  expect(selection.promptChangedInvalidatesMemory).toBe(true);
}

function rejectsDuplicateActiveVersions() {
  const parsed = BrowserAiPromptTemplateRegistrySchema.safeParse({
    ...promptTemplateRegistry(),
    versions: [activeVersionRecord(), duplicateActiveVersionRecord()],
  });

  expect(parsed.success).toBe(false);
}

function returnsManualForUnsupportedModel() {
  const selection = selectBrowserAiPromptTemplate({
    ...selectionRequest(),
    modelRuntimeRef: 'unsupported-local-model-runtime',
  });

  expect(selection.selectionState).toBe('manual-required');
  expect(selection.selectedPromptTemplate).toBeNull();
  expect(selection.degradedStates).toEqual(['model-unsupported']);
}

function doesNotSelectDeprecatedVersions() {
  const selection = selectBrowserAiPromptTemplate({
    ...selectionRequest(),
    registry: {
      ...promptTemplateRegistry(),
      versions: [deprecatedVersionRecord()],
    },
  });

  expect(selection.selectionState).toBe('manual-required');
  expect(selection.selectedPromptTemplate).toBeNull();
  expect(selection.degradedStates).toEqual(['template-missing']);
}

function selectionRequest() {
  return {
    registry: promptTemplateRegistry(),
    requestedTask: 'video-safety',
    modelRuntimeRef: 'local-model-runtime-ref-browser-ai',
    policyVersionRef: 'browser-policy-version-2026-06-03',
    selectedAt: '2026-06-03T03:30:00.000Z',
    auditEvidenceIds: ['prompt-template-selection-audit-video-v2'],
  };
}

function promptTemplateRegistry() {
  return {
    schemaVersion: BrowserAiPromptTemplateVersioningSchemaVersion,
    registryId: 'browser-ai-prompt-registry-v1',
    publishedAt: '2026-06-03T03:29:00.000Z',
    versions: [activeVersionRecord()],
  };
}

function activeVersionRecord() {
  return {
    schemaVersion: BrowserAiPromptTemplateVersioningSchemaVersion,
    template: promptTemplate(),
    status: 'active',
    promptHashRef: 'prompt-template-hash-video-v2',
    changeRef: 'prompt-template-change-video-v2',
    versionedAt: '2026-06-03T03:25:00.000Z',
    validFrom: '2026-06-03T03:25:00.000Z',
    validUntil: null,
    previousPromptTemplateVersion: 'browser-ai-video-safety-template-v1',
    supersededByPromptTemplateVersion: null,
    changeReasons: ['input-field-change', 'risk-taxonomy-change'],
    compatibleModelRuntimeRefs: ['local-model-runtime-ref-browser-ai'],
    policyVersionRefs: ['browser-policy-version-2026-06-03'],
    auditEvidenceIds: ['prompt-template-audit-video-v2'],
    invalidatesMemory: true,
    inputFieldRefsChanged: true,
  };
}

function duplicateActiveVersionRecord() {
  return {
    ...activeVersionRecord(),
    template: {
      ...promptTemplate(),
      promptTemplateVersion: 'browser-ai-video-safety-template-v3',
    },
    promptHashRef: 'prompt-template-hash-video-v3',
    changeRef: 'prompt-template-change-video-v3',
    auditEvidenceIds: ['prompt-template-audit-video-v3'],
  };
}

function deprecatedVersionRecord() {
  return {
    ...activeVersionRecord(),
    status: 'deprecated',
    validUntil: '2026-06-04T03:25:00.000Z',
    supersededByPromptTemplateVersion: 'browser-ai-video-safety-template-v3',
  };
}

function promptTemplate() {
  return {
    promptTemplateId: 'browser-ai-video-safety-template',
    promptTemplateVersion: 'browser-ai-video-safety-template-v2',
    requestedTask: 'video-safety',
    allowedInputFieldRefs: ['url-shape', 'metadata-evidence', 'memory-hit', 'parent-rule', 'schedule-context'],
    rawPromptTextIncluded: false,
    capturesRawPageBody: false,
    capturesTranscriptText: false,
  };
}
