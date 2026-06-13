import { describe, expect, it } from 'vitest';

import {
  ActivityEventKind,
  ActivityEventKindSchema,
  ActivityEvidenceKind,
  ActivityEvidenceKindSchema,
  ActivityObserver,
  ActivityObserverSchema,
  ActivitySubjectKind,
  ActivitySubjectKindSchema,
} from '../../src/kinds';

describe('evidence-domain kind contracts', () => {
  it('parses activity observers from the canonical evidence-domain owner', () => {
    expect(ActivityObserverSchema.parse(ActivityObserver.AgentService)).toBe(ActivityObserver.AgentService);
    expect(ActivityObserverSchema.parse(ActivityObserver.LocalAi)).toBe(ActivityObserver.LocalAi);
  });

  it('parses activity event and subject kinds from exported canonical constants', () => {
    expect(ActivityEventKindSchema.parse(ActivityEventKind.ProcessObserved)).toBe(ActivityEventKind.ProcessObserved);
    expect(ActivitySubjectKindSchema.parse(ActivitySubjectKind.Domain)).toBe(ActivitySubjectKind.Domain);
  });

  it('parses evidence storage kinds from exported canonical constants', () => {
    expect(ActivityEvidenceKindSchema.parse(ActivityEvidenceKind.JournalEntry)).toBe(ActivityEvidenceKind.JournalEntry);
    expect(ActivityEvidenceKindSchema.parse(ActivityEvidenceKind.LocalDbRow)).toBe(ActivityEvidenceKind.LocalDbRow);
  });
});
