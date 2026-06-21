import { describe, expect, it } from 'vitest';
import { ActivityEvidenceKind } from '@ocentra-parent/schema-domain/evidence-kinds';
import {
  ActivityNetworkActivityClassificationSchema,
  ActivityNetworkContractSchemaVersion,
  ActivityNetworkDomainEvidenceSchema,
  ActivityNetworkEvidenceGradeSchema,
  ActivityNetworkFlowEvidenceSchema,
  ActivityNetworkPolicyActionSchema,
} from '../../src/network-contracts';

const EvidenceRef = {
  evidenceId: 'journal-entry-network-contract-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:network-contract-digest',
  uri: null,
} as const;

const DomainEvidence = {
  schemaVersion: ActivityNetworkContractSchemaVersion,
  domainEvidenceId: 'network-domain-evidence-1',
  observedAt: '2026-06-04T03:25:00Z',
  source: 'dns-response',
  attributionStatus: 'domain-observed',
  domainName: 'example.com',
  destinationIp: '93.184.216.34',
  evidenceGrade: 'A',
  confidence: 0.96,
  evidence: [EvidenceRef],
} as const;

const FlowEvidence = {
  schemaVersion: ActivityNetworkContractSchemaVersion,
  flowEvidenceId: 'network-flow-evidence-1',
  observedAt: '2026-06-04T03:25:01Z',
  capabilityStatus: 'available',
  domainAttributionStatus: 'domain-observed',
  processAttributionStatus: 'process-attributed',
  evidenceGrade: 'A',
  confidence: 0.91,
  claimScopes: ['destination-domain', 'destination-ip', 'process-attribution', 'protocol', 'port'],
  unsupportedClaimAttempts: [],
  evidence: [EvidenceRef],
} as const;

const Classification = {
  schemaVersion: ActivityNetworkContractSchemaVersion,
  classificationId: 'network-classification-1',
  classifiedAt: '2026-06-04T03:25:02Z',
  kind: 'video',
  evidenceGrade: 'B',
  confidence: 0.82,
  uncertaintyReason: null,
  evidenceIds: ['journal-entry-network-contract-1'],
  evidence: [EvidenceRef],
} as const;

const ManualCapability = {
  capabilityId: 'host-network-domain-filter',
  state: 'manual-required',
  proofRefs: [],
  manualRequiredReason: 'Host DNS/firewall adapter apply and rollback proof is missing.',
} as const;

describe('network contract boundary Effect schemas', () => {
  it('accepts flow, domain, classification, and manual-required policy contracts', () => {
    const flow = ActivityNetworkFlowEvidenceSchema.parse(FlowEvidence);
    const domain = ActivityNetworkDomainEvidenceSchema.parse(DomainEvidence);
    const classification = ActivityNetworkActivityClassificationSchema.parse(Classification);
    const action = ActivityNetworkPolicyActionSchema.parse({
      schemaVersion: ActivityNetworkContractSchemaVersion,
      actionId: 'network-policy-action-1',
      decidedAt: '2026-06-04T03:25:03Z',
      mode: 'manual-required',
      action: 'ask-parent',
      evidenceGrade: 'B',
      policyDecisionRef: 'policy-decision-network-dry-run-1',
      adapterCapability: ManualCapability,
      adapterCallAuthorized: false,
      evidence: [EvidenceRef],
    });

    expect(flow.claimScopes).toContain('destination-domain');
    expect(domain.domainName).toBe('example.com');
    expect(classification.kind).toBe('video');
    expect(action.adapterCallAuthorized).toBe(false);
  });

  it('rejects unsupported exact-content claims from network-only evidence', () => {
    const exactUrlAttempt = ActivityNetworkFlowEvidenceSchema.safeParse({
      ...FlowEvidence,
      unsupportedClaimAttempts: ['exact-url'],
    });
    const decryptedPayloadAttempt = ActivityNetworkFlowEvidenceSchema.safeParse({
      ...FlowEvidence,
      unsupportedClaimAttempts: ['decrypted-payload'],
    });

    expect(exactUrlAttempt.success).toBe(false);
    expect(decryptedPayloadAttempt.success).toBe(false);
  });
});

describe('network domain evidence source contracts', () => {
  it('validates domain attribution source states', () => {
    const ipOnlyWithDomain = ActivityNetworkDomainEvidenceSchema.safeParse({
      ...DomainEvidence,
      source: 'ip-only',
      attributionStatus: 'ip-only',
      domainName: 'example.com',
    });
    const unavailableWithStrongGrade = ActivityNetworkDomainEvidenceSchema.safeParse({
      ...DomainEvidence,
      source: 'unavailable',
      attributionStatus: 'unavailable',
      domainName: null,
      evidenceGrade: 'A',
    });
    const validIpOnly = ActivityNetworkDomainEvidenceSchema.safeParse({
      ...DomainEvidence,
      source: 'ip-only',
      attributionStatus: 'ip-only',
      domainName: null,
      evidenceGrade: 'C',
    });

    expect(ipOnlyWithDomain.success).toBe(false);
    expect(unavailableWithStrongGrade.success).toBe(false);
    expect(validIpOnly.success).toBe(true);
  });
});

describe('network classification contracts', () => {
  it('keeps unknown classifications uncertain and lower-grade', () => {
    const unknownWithoutReason = ActivityNetworkActivityClassificationSchema.safeParse({
      ...Classification,
      kind: 'unknown',
      evidenceGrade: 'B',
      uncertaintyReason: null,
    });
    const unknownWithReason = ActivityNetworkActivityClassificationSchema.safeParse({
      ...Classification,
      kind: 'unknown',
      evidenceGrade: 'C',
      uncertaintyReason: 'Only IP and byte counters were observed.',
    });

    expect(unknownWithoutReason.success).toBe(false);
    expect(unknownWithReason.success).toBe(true);
  });
});

describe('network policy action contracts', () => {
  it('requires policy and proved capability before authorizing adapter calls', () => {
    const unauthorizedBlock = ActivityNetworkPolicyActionSchema.safeParse({
      schemaVersion: ActivityNetworkContractSchemaVersion,
      actionId: 'network-policy-action-2',
      decidedAt: '2026-06-04T03:25:04Z',
      mode: 'dry-run',
      action: 'block',
      evidenceGrade: 'B',
      policyDecisionRef: null,
      adapterCapability: ManualCapability,
      adapterCallAuthorized: true,
      evidence: [EvidenceRef],
    });
    const provedBlock = ActivityNetworkPolicyActionSchema.safeParse({
      schemaVersion: ActivityNetworkContractSchemaVersion,
      actionId: 'network-policy-action-3',
      decidedAt: '2026-06-04T03:25:05Z',
      mode: 'apply-ready',
      action: 'block',
      evidenceGrade: 'A',
      policyDecisionRef: 'policy-decision-network-apply-1',
      adapterCapability: {
        capabilityId: 'host-network-domain-filter',
        state: 'proved-available',
        proofRefs: [EvidenceRef],
        manualRequiredReason: null,
      },
      adapterCallAuthorized: true,
      evidence: [EvidenceRef],
    });

    expect(unauthorizedBlock.success).toBe(false);
    expect(provedBlock.success).toBe(true);
  });

  it('blocks apply-ready adapter authorization for weak or unavailable evidence grades', () => {
    expect(authorizedApplyReadyAction('B').success).toBe(false);
    expect(authorizedApplyReadyAction('C').success).toBe(false);
    expect(authorizedApplyReadyAction('D').success).toBe(false);
  });

  it('rejects evidence grades outside the A/B/C/D model', () => {
    expect(ActivityNetworkEvidenceGradeSchema.safeParse('E').success).toBe(false);
  });
});

function authorizedApplyReadyAction(evidenceGrade: 'B' | 'C' | 'D') {
  return ActivityNetworkPolicyActionSchema.safeParse({
    schemaVersion: ActivityNetworkContractSchemaVersion,
    actionId: 'network-policy-action-weak-evidence',
    decidedAt: '2026-06-04T03:25:06Z',
    mode: 'apply-ready',
    action: 'block',
    evidenceGrade,
    policyDecisionRef: 'policy-decision-network-apply-weak-evidence',
    adapterCapability: {
      capabilityId: 'host-network-domain-filter',
      state: 'proved-available',
      proofRefs: [EvidenceRef],
      manualRequiredReason: null,
    },
    adapterCallAuthorized: true,
    evidence: [EvidenceRef],
  });
}
