import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import { ActivityEvidenceRefSchema } from './evidence-contracts';
import { ActivityTimestampSchema } from './evidence-primitives';
import {
  ScreenEvidenceOcrSnippetTextSchema,
  ScreenEvidenceSnippetLimitSchema,
} from './screen-evidence-primitives';
import { ScreenOcrTextSnippetSchema } from './screen-evidence-result';
import {
  ScreenRedactionModeSchema,
  ScreenRedactionNoteSchema,
} from './screen-evidence-states';

export const ScreenOcrRedactionSchemaVersion = 1;
export const ScreenOcrRedactionMaxSnippetLimit = 5;

const RequiredTrue = Schema.Literal(true);
const RequiredFalse = Schema.Literal(false);
const ScreenEvidenceOcrSnippetTextParser = withParser(ScreenEvidenceOcrSnippetTextSchema);
const RedactionSnippetLimitSchema = ScreenEvidenceSnippetLimitSchema.pipe(
  Schema.filter(
    (value) =>
      value <= ScreenOcrRedactionMaxSnippetLimit ||
      'Expected OCR redaction proof to keep snippet retention at or below the screen OCR worker cap'
  )
);

export const ScreenOcrTextRetentionModeSchema = withParser(
  Schema.Literal('disabled', 'redactedSnippets', 'boundedSnippets')
);

export const ScreenOcrSensitiveTextKindSchema = withParser(
  Schema.Literal('credentialLikeText', 'emailLikeText', 'phoneLikeText')
);

export const ScreenOcrRedactionPolicySchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenOcrRedactionSchemaVersion),
    policyId: NonEmptyStringSchema,
    updatedAt: ActivityTimestampSchema,
    ocrTextEnabled: Schema.Boolean,
    snippetLimit: RedactionSnippetLimitSchema,
    redactionMode: ScreenRedactionModeSchema,
    textRetentionMode: ScreenOcrTextRetentionModeSchema,
    credentialSuppressionEnabled: RequiredTrue,
    piiRedactionEnabled: Schema.Boolean,
    parentControlled: RequiredTrue,
    rawTextRetentionAllowed: RequiredFalse,
  }).pipe(
    Schema.filter(
      (value) =>
        value.ocrTextEnabled ||
        (value.snippetLimit === 0 && value.textRetentionMode === 'disabled') ||
        'Expected disabled OCR text to retain no snippets'
    ),
    Schema.filter(
      (value) =>
        !value.ocrTextEnabled ||
        (value.snippetLimit > 0 &&
          value.redactionMode !== 'disabled' &&
          value.textRetentionMode !== 'disabled') ||
        'Expected enabled OCR text to use an explicit redaction mode, positive snippet limit, and retention mode'
    ),
    Schema.filter(
      (value) =>
        value.redactionMode !== 'localSensitiveText' ||
        value.piiRedactionEnabled ||
        'Expected local sensitive text mode to enable PII redaction'
    )
  )
);

export const ScreenOcrCandidateTextLineSchema = withParser(
  Schema.Struct({
    text: ScreenEvidenceOcrSnippetTextSchema,
    confidence: Schema.Number.pipe(Schema.between(0, 1)),
    evidenceRefs: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const ScreenOcrSuppressedTextLineSchema = withParser(
  Schema.Struct({
    sensitiveKind: ScreenOcrSensitiveTextKindSchema,
    evidenceRefs: Schema.Array(ActivityEvidenceRefSchema),
    redactionNote: ScreenRedactionNoteSchema,
  })
);

export const ScreenOcrRedactionResultSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenOcrRedactionSchemaVersion),
    policyId: NonEmptyStringSchema,
    processedAt: ActivityTimestampSchema,
    snippets: Schema.Array(ScreenOcrTextSnippetSchema),
    suppressed: Schema.Array(ScreenOcrSuppressedTextLineSchema),
    redactionNotes: Schema.Array(ScreenRedactionNoteSchema),
    snippetLimit: RedactionSnippetLimitSchema,
    rawTextRetained: RequiredFalse,
    rawImageRetained: RequiredFalse,
    remoteAiUsed: RequiredFalse,
  }).pipe(
    Schema.filter(
      (value) =>
        value.snippets.length <= value.snippetLimit ||
        'Expected redacted OCR snippets to stay within the parent-controlled snippet limit'
    ),
    Schema.filter(
      (value) =>
        value.suppressed.length === 0 ||
        value.redactionNotes.includes('credentialLikeTextRedacted') ||
        value.redactionNotes.includes('piiLikeTextRedacted') ||
        'Expected suppressed sensitive OCR text to leave a visible redaction note'
    )
  )
);

export const ScreenOcrRedactionProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenOcrRedactionSchemaVersion),
    proofId: NonEmptyStringSchema,
    proofTier: Schema.Literal('P2_CONTRACT_SCREEN_OCR_REDACTION'),
    policy: ScreenOcrRedactionPolicySchema,
    result: ScreenOcrRedactionResultSchema,
    credentialSuppressed: RequiredTrue,
    piiRedacted: RequiredTrue,
    disabledStateProved: RequiredTrue,
    localOnly: RequiredTrue,
    rawTextRetained: RequiredFalse,
    rawImageRetained: RequiredFalse,
    remoteAiUsed: RequiredFalse,
    portalRuntimeClaimed: RequiredFalse,
    servicePersistenceClaimed: RequiredFalse,
  }).pipe(
    Schema.filter(
      (value) =>
        value.policy.policyId === value.result.policyId ||
        'Expected OCR redaction proof policy and result to reference the same policy id'
    )
  )
);

type ScreenOcrRedactionNote = Infer<typeof ScreenRedactionNoteSchema>;

const CredentialPattern = /\b(password|passcode|otp|token|secret|api\s*key)\b/i;
const EmailPattern = /\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b/i;
const PhonePattern = /\b(?:\+?1[-.\s]?)?\(?[0-9]{3}\)?[-.\s]?[0-9]{3}[-.\s]?[0-9]{4}\b/;

export function redactScreenOcrText(input: {
  readonly policy: ScreenOcrRedactionPolicy;
  readonly processedAt: unknown;
  readonly lines: readonly unknown[];
}) {
  const policy = ScreenOcrRedactionPolicySchema.parse(input.policy);
  if (!policy.ocrTextEnabled || policy.textRetentionMode === 'disabled') {
    return ScreenOcrRedactionResultSchema.parse({
      schemaVersion: ScreenOcrRedactionSchemaVersion,
      policyId: policy.policyId,
      processedAt: input.processedAt,
      snippets: [],
      suppressed: [],
      redactionNotes: ['ocrDisabled'],
      snippetLimit: policy.snippetLimit,
      rawTextRetained: false,
      rawImageRetained: false,
      remoteAiUsed: false,
    });
  }

  const snippets: Array<Infer<typeof ScreenOcrTextSnippetSchema>> = [];
  const suppressed: Array<ScreenOcrSuppressedTextLine> = [];
  const redactionNotes = new Set<ScreenOcrRedactionNote>();

  for (const candidateLine of input.lines) {
    const line = ScreenOcrCandidateTextLineSchema.parse(candidateLine);
    if (snippets.length >= policy.snippetLimit) {
      break;
    }

    if (CredentialPattern.test(line.text)) {
      suppressed.push({
        sensitiveKind: 'credentialLikeText',
        evidenceRefs: line.evidenceRefs,
        redactionNote: 'credentialLikeTextRedacted',
      });
      redactionNotes.add('credentialLikeTextRedacted');
      continue;
    }

    const redactedText = redactPiiText(line.text, policy);
    if (redactedText !== line.text) {
      redactionNotes.add('piiLikeTextRedacted');
    }

    snippets.push(
      ScreenOcrTextSnippetSchema.parse({
        text: redactedText,
        confidence: line.confidence,
        evidenceRefs: line.evidenceRefs,
      })
    );
  }

  if (snippets.length === 0 && redactionNotes.size === 0) {
    redactionNotes.add('noTextExtracted');
  }

  return ScreenOcrRedactionResultSchema.parse({
    schemaVersion: ScreenOcrRedactionSchemaVersion,
    policyId: policy.policyId,
    processedAt: input.processedAt,
    snippets,
    suppressed,
    redactionNotes: [...redactionNotes],
    snippetLimit: policy.snippetLimit,
    rawTextRetained: false,
    rawImageRetained: false,
    remoteAiUsed: false,
  });
}

function redactPiiText(
  value: ScreenEvidenceOcrSnippetText,
  policy: ScreenOcrRedactionPolicy
): ScreenEvidenceOcrSnippetText {
  if (!policy.piiRedactionEnabled || policy.textRetentionMode !== 'redactedSnippets') {
    return value;
  }
  return ScreenEvidenceOcrSnippetTextParser.parse(
    value
      .replace(EmailPattern, '[redacted-email]')
      .replace(PhonePattern, '[redacted-phone]')
  );
}

export type ScreenOcrTextRetentionMode = Infer<typeof ScreenOcrTextRetentionModeSchema>;
export type ScreenOcrSensitiveTextKind = Infer<typeof ScreenOcrSensitiveTextKindSchema>;
export type ScreenOcrRedactionPolicy = Infer<typeof ScreenOcrRedactionPolicySchema>;
export type ScreenOcrCandidateTextLine = Infer<typeof ScreenOcrCandidateTextLineSchema>;
export type ScreenOcrSuppressedTextLine = Infer<typeof ScreenOcrSuppressedTextLineSchema>;
export type ScreenOcrRedactionResult = Infer<typeof ScreenOcrRedactionResultSchema>;
export type ScreenOcrRedactionProof = Infer<typeof ScreenOcrRedactionProofSchema>;
export type ScreenEvidenceOcrSnippetText = Infer<typeof ScreenEvidenceOcrSnippetTextSchema>;
