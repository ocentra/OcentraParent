import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

export const ActivityObserverSchema = withParser(
  Schema.Literal(
    'agent-service',
    'windows-process',
    'windows-window',
    'windows-network',
    'managed-browser-bridge',
    'browser-extension',
    'local-ai'
  )
);

export const ActivityEventKindSchema = withParser(
  Schema.Literal(
    'activity.process.observed',
    'activity.window.focused',
    'activity.domain.observed',
    'activity.url.observed',
    'activity.video.observed',
    'activity.device.idle-state-observed'
  )
);

export const ActivitySubjectKindSchema = withParser(
  Schema.Literal('process', 'window', 'domain', 'url', 'video', 'device')
);

export const ActivityEvidenceKindSchema = withParser(
  Schema.Literal('journal-entry', 'screenshot', 'storage-object', 'local-db-row')
);

export type ActivityObserver = Infer<typeof ActivityObserverSchema>;
export type ActivityEventKind = Infer<typeof ActivityEventKindSchema>;
export type ActivitySubjectKind = Infer<typeof ActivitySubjectKindSchema>;
export type ActivityEvidenceKind = Infer<typeof ActivityEvidenceKindSchema>;

export const ActivityObserver = {
  AgentService: ActivityObserverSchema.parse('agent-service'),
  WindowsProcess: ActivityObserverSchema.parse('windows-process'),
  WindowsWindow: ActivityObserverSchema.parse('windows-window'),
  WindowsNetwork: ActivityObserverSchema.parse('windows-network'),
  ManagedBrowserBridge: ActivityObserverSchema.parse('managed-browser-bridge'),
  BrowserExtension: ActivityObserverSchema.parse('browser-extension'),
  LocalAi: ActivityObserverSchema.parse('local-ai'),
} as const;

export const ActivityEventKind = {
  ProcessObserved: ActivityEventKindSchema.parse('activity.process.observed'),
  WindowFocused: ActivityEventKindSchema.parse('activity.window.focused'),
  DomainObserved: ActivityEventKindSchema.parse('activity.domain.observed'),
  UrlObserved: ActivityEventKindSchema.parse('activity.url.observed'),
  VideoObserved: ActivityEventKindSchema.parse('activity.video.observed'),
  DeviceIdleStateObserved: ActivityEventKindSchema.parse('activity.device.idle-state-observed'),
} as const;

export const ActivitySubjectKind = {
  Process: ActivitySubjectKindSchema.parse('process'),
  Window: ActivitySubjectKindSchema.parse('window'),
  Domain: ActivitySubjectKindSchema.parse('domain'),
  Url: ActivitySubjectKindSchema.parse('url'),
  Video: ActivitySubjectKindSchema.parse('video'),
  Device: ActivitySubjectKindSchema.parse('device'),
} as const;

export const ActivityEvidenceKind = {
  JournalEntry: ActivityEvidenceKindSchema.parse('journal-entry'),
  Screenshot: ActivityEvidenceKindSchema.parse('screenshot'),
  StorageObject: ActivityEvidenceKindSchema.parse('storage-object'),
  LocalDbRow: ActivityEvidenceKindSchema.parse('local-db-row'),
} as const;
