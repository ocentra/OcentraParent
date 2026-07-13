struct LiteralEntry {
    key: &'static str,
    value: &'static str,
}

const ACTIVITY_OBSERVER_ENTRIES: &[LiteralEntry] = &[
    LiteralEntry {
        key: "AgentService",
        value: "agent-service",
    },
    LiteralEntry {
        key: "WindowsProcess",
        value: "windows-process",
    },
    LiteralEntry {
        key: "WindowsWindow",
        value: "windows-window",
    },
    LiteralEntry {
        key: "WindowsNetwork",
        value: "windows-network",
    },
    LiteralEntry {
        key: "ManagedBrowserBridge",
        value: "managed-browser-bridge",
    },
    LiteralEntry {
        key: "BrowserExtension",
        value: "browser-extension",
    },
    LiteralEntry {
        key: "LocalAi",
        value: "local-ai",
    },
];

const ACTIVITY_SUBJECT_KIND_ENTRIES: &[LiteralEntry] = &[
    LiteralEntry {
        key: "Process",
        value: "process",
    },
    LiteralEntry {
        key: "Window",
        value: "window",
    },
    LiteralEntry {
        key: "Domain",
        value: "domain",
    },
    LiteralEntry {
        key: "Url",
        value: "url",
    },
    LiteralEntry {
        key: "Video",
        value: "video",
    },
    LiteralEntry {
        key: "Device",
        value: "device",
    },
    LiteralEntry {
        key: "Intervention",
        value: "intervention",
    },
];

const ACTIVITY_EVIDENCE_KIND_ENTRIES: &[LiteralEntry] = &[
    LiteralEntry {
        key: "JournalEntry",
        value: "journal-entry",
    },
    LiteralEntry {
        key: "Screenshot",
        value: "screenshot",
    },
    LiteralEntry {
        key: "StorageObject",
        value: "storage-object",
    },
    LiteralEntry {
        key: "LocalDbRow",
        value: "local-db-row",
    },
];

pub fn evidence_kinds_typescript() -> String {
    format!(
        "{}{}{}{}{}{}",
        TYPESCRIPT_HEADER,
        literal_definition("ActivityObserverDefinition", ACTIVITY_OBSERVER_ENTRIES),
        "const ActivityEventKindDefinition = defineLiteralKindGroup(ActivityEventKindLiteralSource);\n\n",
        literal_definition(
            "ActivitySubjectKindDefinition",
            ACTIVITY_SUBJECT_KIND_ENTRIES
        ),
        literal_definition(
            "ActivityEvidenceKindDefinition",
            ACTIVITY_EVIDENCE_KIND_ENTRIES
        ),
        TYPESCRIPT_EXPORTS
    )
}

fn literal_definition(name: &str, entries: &[LiteralEntry]) -> String {
    let body = entries
        .iter()
        .map(|entry| format!("  {}: '{}',\n", entry.key, entry.value))
        .collect::<String>();

    format!("const {name} = defineLiteralKindGroup({{\n{body}}} as const);\n\n")
}

const TYPESCRIPT_HEADER: &str = r#"/* generated from crates/schema/src/evidence_kinds_ts.rs */

import { type Infer, Schema, withParser } from './effect';
import { ActivityEventKindLiteral as ActivityEventKindLiteralSource } from './generated-activity-event-kind';

function defineLiteralKindGroup<const TLiteral extends Record<string, string>>(literal: TLiteral) {
  const schema = withParser(
    Schema.Literal(...(Object.values(literal) as [TLiteral[keyof TLiteral], ...TLiteral[keyof TLiteral][]]))
  );
  const parsed = Object.fromEntries(Object.entries(literal).map(([key, value]) => [key, schema.parse(value)])) as {
    readonly [K in keyof TLiteral]: TLiteral[K];
  };

  return {
    literal,
    schema,
    parsed,
  } as const;
}

"#;

const TYPESCRIPT_EXPORTS: &str = r#"export const ActivityObserverLiteral = ActivityObserverDefinition.literal;
export const ActivityEventKindLiteral = ActivityEventKindDefinition.literal;
export const ActivitySubjectKindLiteral = ActivitySubjectKindDefinition.literal;
export const ActivityEvidenceKindLiteral = ActivityEvidenceKindDefinition.literal;

export const ActivityObserverSchema = ActivityObserverDefinition.schema;
export const ActivityEventKindSchema = ActivityEventKindDefinition.schema;
export const ActivitySubjectKindSchema = ActivitySubjectKindDefinition.schema;
export const ActivityEvidenceKindSchema = ActivityEvidenceKindDefinition.schema;

export type ActivityObserver = Infer<typeof ActivityObserverSchema>;
export type ActivityEventKind = Infer<typeof ActivityEventKindSchema>;
export type ActivitySubjectKind = Infer<typeof ActivitySubjectKindSchema>;
export type ActivityEvidenceKind = Infer<typeof ActivityEvidenceKindSchema>;

export const ActivityObserver = ActivityObserverDefinition.parsed;
export const ActivityEventKind = ActivityEventKindDefinition.parsed;
export const ActivitySubjectKind = ActivitySubjectKindDefinition.parsed;
export const ActivityEvidenceKind = ActivityEvidenceKindDefinition.parsed;
"#;
