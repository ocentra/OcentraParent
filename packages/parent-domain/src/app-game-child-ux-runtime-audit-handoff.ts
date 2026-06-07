import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameChildUxCardSchema,
  AppGameChildUxReasonRefSchema,
  AppGameChildUxStateIdSchema,
  AppGameChildUxStatusRefSchema,
  AppGameChildUxTargetRefSchema,
} from './app-game-child-facing-ux';
import {
  AppGameChildUxRuntimeAuditHandoffNoClaimFlags,
  AppGameChildUxRuntimeAuditHandoffState,
  RequiredAppGameChildUxRuntimeAuditHandoffNonClaims,
  appGameChildUxRuntimeAuditHandoffCountsMatch,
  appGameChildUxRuntimeAuditHandoffHasNoRuntimeClaims,
  appGameChildUxRuntimeAuditStateForCard,
  appGameChildUxTargetKindToDomain,
} from './app-game-child-ux-runtime-audit-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const ChildUxRuntimeAuditHandoffText = Schema.String.pipe(Schema.minLength(1));

export const AppGameChildUxRuntimeAuditHandoffIdSchema = ChildUxRuntimeAuditHandoffText.pipe(
  Schema.brand('AppGameChildUxRuntimeAuditHandoffId')
);
export const AppGameChildUxRuntimeAuditHandoffRowIdSchema = ChildUxRuntimeAuditHandoffText.pipe(
  Schema.brand('AppGameChildUxRuntimeAuditHandoffRowId')
);
export const AppGameChildUxRuntimeAuditProofRefSchema = ChildUxRuntimeAuditHandoffText.pipe(
  Schema.brand('AppGameChildUxRuntimeAuditProofRef')
);
export const AppGameChildUxRuntimeAuditContractRefSchema = ChildUxRuntimeAuditHandoffText.pipe(
  Schema.brand('AppGameChildUxRuntimeAuditContractRef')
);

const AppGameChildUxRuntimeAuditHandoffStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameChildUxRuntimeAuditHandoffState))
);
const AppGameChildUxRuntimeAuditHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameChildUxRuntimeAuditHandoffNonClaims)
);
const AppGameChildUxRuntimeAuditTargetDomainSchema = withParser(Schema.Literal('native-app', 'native-game'));

export const AppGameChildUxRuntimeAuditHandoffOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    childUxRuntimeAuditHandoffId: AppGameChildUxRuntimeAuditHandoffIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameChildUxRuntimeAuditContractRefSchema),
    runtimeAuditProofRefs: Schema.Array(AppGameChildUxRuntimeAuditProofRefSchema),
  }).pipe(
    Schema.filter(
      (options) =>
        (options.sourceContractRefs.length > 0 && options.runtimeAuditProofRefs.length > 0) ||
        'Expected child UX runtime audit handoff options to cite source contracts and future runtime audit proof refs'
    )
  )
);

const AppGameChildUxRuntimeAuditHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameChildUxRuntimeAuditHandoffRowIdSchema,
  sourceChildUxStateId: AppGameChildUxStateIdSchema,
  targetDomain: AppGameChildUxRuntimeAuditTargetDomainSchema,
  targetRef: AppGameChildUxTargetRefSchema,
  runtimeAuditHandoffState: AppGameChildUxRuntimeAuditHandoffStateSchema,
  inheritedChildReasonReferences: Schema.Array(AppGameChildUxReasonRefSchema),
  inheritedChildStatusReferences: Schema.Array(AppGameChildUxStatusRefSchema),
  requiredRuntimeAuditProofRefs: Schema.Array(AppGameChildUxRuntimeAuditProofRefSchema),
  ...Object.fromEntries(
    Object.keys(AppGameChildUxRuntimeAuditHandoffNoClaimFlags).map((key) => [key, Schema.Literal(false)])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameChildUxRuntimeAuditHandoffRowSchema = withParser(
  AppGameChildUxRuntimeAuditHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.runtimeAuditHandoffState !== AppGameChildUxRuntimeAuditHandoffState.RuntimeAuditReady ||
        (row.inheritedChildReasonReferences.length > 0 &&
          row.inheritedChildStatusReferences.length > 0 &&
          row.requiredRuntimeAuditProofRefs.length > 0) ||
        'Expected runtime-audit-ready rows to cite child reason refs, child status refs, and runtime audit proof refs'
    )
  ).pipe(
    Schema.filter(
      (row) =>
        row.runtimeAuditHandoffState !== AppGameChildUxRuntimeAuditHandoffState.ManualRequiredNoAdapter ||
        row.requiredRuntimeAuditProofRefs.length === 0 ||
        'Expected manual-required child UX rows to avoid future runtime audit proof claims'
    )
  )
);

const AppGameChildUxRuntimeAuditHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  childUxRuntimeAuditHandoffId: AppGameChildUxRuntimeAuditHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameChildUxRuntimeAuditContractRefSchema),
  rows: Schema.Array(AppGameChildUxRuntimeAuditHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  runtimeAuditReadyCount: Schema.Number,
  blockedMissingChildReasonCount: Schema.Number,
  blockedMissingChildStatusCount: Schema.Number,
  manualRequiredNoAdapterCount: Schema.Number,
  runtimeAuditHandoffNonClaims: Schema.Array(AppGameChildUxRuntimeAuditHandoffNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(AppGameChildUxRuntimeAuditHandoffNoClaimFlags).map((key) => [key, Schema.Literal(false)])
  ),
});

export const AppGameChildUxRuntimeAuditHandoffSchema = withParser(
  AppGameChildUxRuntimeAuditHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameChildUxRuntimeAuditHandoffCountsMatch(handoff) ||
        'Expected child UX runtime audit handoff counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameChildUxRuntimeAuditHandoffHasNoRuntimeClaims(handoff) ||
        'Expected child UX runtime audit handoff to keep runtime, adapter, platform, and private diagnostic claims false'
    )
  )
);

export function buildAppGameChildUxRuntimeAuditHandoff(
  options: Infer<typeof AppGameChildUxRuntimeAuditHandoffOptionsSchema>,
  childUxCards: ReadonlyArray<Infer<typeof AppGameChildUxCardSchema>>
): Infer<typeof AppGameChildUxRuntimeAuditHandoffSchema> {
  const parsedOptions = AppGameChildUxRuntimeAuditHandoffOptionsSchema.parse(options);
  const parsedCards = childUxCards.map((card) => AppGameChildUxCardSchema.parse(card));
  const rows = parsedCards.map((card, index) => buildChildUxRuntimeAuditHandoffRow(parsedOptions, card, index));

  return AppGameChildUxRuntimeAuditHandoffSchema.parse({
    schemaVersion: parsedOptions.schemaVersion,
    childUxRuntimeAuditHandoffId: parsedOptions.childUxRuntimeAuditHandoffId,
    generatedAt: parsedOptions.generatedAt,
    sourceContractRefs: parsedOptions.sourceContractRefs,
    rows,
    nativeAppRowCount: rows.filter((row) => row.targetDomain === 'native-app').length,
    nativeGameRowCount: rows.filter((row) => row.targetDomain === 'native-game').length,
    runtimeAuditReadyCount: rows.filter(
      (row) => row.runtimeAuditHandoffState === AppGameChildUxRuntimeAuditHandoffState.RuntimeAuditReady
    ).length,
    blockedMissingChildReasonCount: rows.filter(
      (row) => row.runtimeAuditHandoffState === AppGameChildUxRuntimeAuditHandoffState.BlockedMissingChildReason
    ).length,
    blockedMissingChildStatusCount: rows.filter(
      (row) => row.runtimeAuditHandoffState === AppGameChildUxRuntimeAuditHandoffState.BlockedMissingChildStatus
    ).length,
    manualRequiredNoAdapterCount: rows.filter(
      (row) => row.runtimeAuditHandoffState === AppGameChildUxRuntimeAuditHandoffState.ManualRequiredNoAdapter
    ).length,
    runtimeAuditHandoffNonClaims: RequiredAppGameChildUxRuntimeAuditHandoffNonClaims,
    ...AppGameChildUxRuntimeAuditHandoffNoClaimFlags,
  });
}

function buildChildUxRuntimeAuditHandoffRow(
  options: Infer<typeof AppGameChildUxRuntimeAuditHandoffOptionsSchema>,
  card: Infer<typeof AppGameChildUxCardSchema>,
  index: number
): Infer<typeof AppGameChildUxRuntimeAuditHandoffRowSchema> {
  const runtimeAuditHandoffState = appGameChildUxRuntimeAuditStateForCard(card);
  return AppGameChildUxRuntimeAuditHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${options.childUxRuntimeAuditHandoffId}-row-${index + 1}`,
    sourceChildUxStateId: card.childUxStateId,
    targetDomain: appGameChildUxTargetKindToDomain(card.target.targetKind),
    targetRef: card.target.targetRef,
    runtimeAuditHandoffState,
    inheritedChildReasonReferences: card.childReasonReferences,
    inheritedChildStatusReferences: card.childStatusReferences,
    requiredRuntimeAuditProofRefs:
      runtimeAuditHandoffState === AppGameChildUxRuntimeAuditHandoffState.RuntimeAuditReady
        ? options.runtimeAuditProofRefs
        : [],
    ...AppGameChildUxRuntimeAuditHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

export type AppGameChildUxRuntimeAuditHandoff = Infer<typeof AppGameChildUxRuntimeAuditHandoffSchema>;
export type AppGameChildUxRuntimeAuditHandoffRow = Infer<typeof AppGameChildUxRuntimeAuditHandoffRowSchema>;
