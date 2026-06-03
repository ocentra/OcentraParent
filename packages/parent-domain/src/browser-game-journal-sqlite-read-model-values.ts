import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyBrowserGameReadModelText = Schema.String.pipe(Schema.minLength(1));

export const BrowserGameJournalSqliteReadModelSchemaVersionSchema = withParser(
  Schema.Literal('browser-game-journal-sqlite-read-model-contract')
);

export const BrowserGameJournalSqliteReadModelIdSchema = withParser(
  NonEmptyBrowserGameReadModelText.pipe(Schema.brand('BrowserGameJournalSqliteReadModelId'))
);

export const BrowserGameJournalSqliteReadModelRowIdSchema = withParser(
  NonEmptyBrowserGameReadModelText.pipe(Schema.brand('BrowserGameJournalSqliteReadModelRowId'))
);

export const BrowserGameReadModelSourceKindSchema = withParser(
  Schema.Literal(
    'managed-browser-evidence',
    'app-game-session-report',
    'adapter-plan-audit',
    'manual-required',
    'unavailable'
  )
);

export const BrowserGameReadModelStorageStateSchema = withParser(
  Schema.Literal('journal-replayed', 'read-model-present', 'manual-required', 'unavailable')
);

export const BrowserGameReadModelRowStateSchema = withParser(
  Schema.Literal('partial-proof', 'manual-required', 'unavailable')
);

export const BrowserGameReadModelReasonSchema = withParser(
  Schema.Literal(
    'browser-journal-replay-proof-present',
    'sqlite-read-model-proof-present',
    'app-game-session-read-model-present',
    'adapter-audit-ref-present',
    'browser-game-exact-proof-manual-required',
    'cloud-gaming-read-model-manual-required',
    'native-game-control-unavailable',
    'unmanaged-browser-exact-url-unavailable'
  )
);

export type BrowserGameReadModelSourceKind = Infer<typeof BrowserGameReadModelSourceKindSchema>;
