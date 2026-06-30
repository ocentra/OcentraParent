import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  ActivityCaptureCapabilityStatusSchema,
  ActivityDomainAttributionStatusSchema,
  ActivityNetworkProtocolSchema,
  ActivityNetworkTcpStateSchema,
  ActivityProcessAttributionStatusSchema,
} from '@ocentra-parent/schema-domain/activity-capture';
import {
  ActivityEventKindSchema,
  ActivityObserverSchema,
  ActivitySubjectKindSchema,
} from '@ocentra-parent/schema-domain/evidence-kinds';
import {
  ActivityNetworkAdapterIdSchema,
  ActivityNetworkDomainNameSchema,
  ActivityNetworkEndpointAddressSchema,
  ActivityNetworkProcessNameSchema,
} from '@ocentra-parent/schema-domain/network-flow';
import {
  decodeActivityDeviceId,
  decodeActivityEvidenceDigest,
  decodeActivityEvidenceId,
  decodeActivityEventId,
  decodeActivityPlatform,
  decodeActivitySubjectId,
  decodeActivitySubjectName,
} from '@ocentra-parent/schema-domain/evidence-primitives';

type NetworkEvidenceDrawerProofFixture = {
  readonly eventId: string;
  readonly evidenceId: string;
  readonly journalEvidenceId: string;
  readonly evidenceDigest: string;
  readonly journalEvidenceDigest: string;
  readonly deviceId: string;
  readonly platform: string;
  readonly observer: string;
  readonly kind: string;
  readonly subjectKind: string;
  readonly subjectId: string;
  readonly subjectDisplayName: string;
  readonly fields: {
    readonly capabilityStatus: string;
    readonly adapterId: string;
    readonly networkProtocol: string;
    readonly tcpState: string;
    readonly localIp: string;
    readonly localPort: number;
    readonly destinationIp: string;
    readonly destinationPort: number;
    readonly destinationDomain: string;
    readonly domainAttributionStatus: string;
    readonly processAttributionStatus: string;
    readonly pid: number;
    readonly processName: string;
  };
  readonly expected: {
    readonly domainEvidenceRef: string;
    readonly processRef: string;
  };
};

const fixturePath = path.join(
  path.dirname(fileURLToPath(import.meta.url)),
  '../../../../../scripts/test/fixtures/network-evidence-drawer-proof.json'
);
const fixture = JSON.parse(readFileSync(fixturePath, 'utf8')) as NetworkEvidenceDrawerProofFixture;

export const NetworkEvidenceDrawerProof = Object.freeze({
  ...fixture,
  eventId: decodeActivityEventId(fixture.eventId),
  evidenceId: decodeActivityEvidenceId(fixture.evidenceId),
  journalEvidenceId: decodeActivityEvidenceId(fixture.journalEvidenceId),
  evidenceDigest: decodeActivityEvidenceDigest(fixture.evidenceDigest),
  journalEvidenceDigest: decodeActivityEvidenceDigest(fixture.journalEvidenceDigest),
  deviceId: decodeActivityDeviceId(fixture.deviceId),
  platform: decodeActivityPlatform(fixture.platform),
  observer: ActivityObserverSchema.parse(fixture.observer),
  kind: ActivityEventKindSchema.parse(fixture.kind),
  subjectKind: ActivitySubjectKindSchema.parse(fixture.subjectKind),
  subjectId: decodeActivitySubjectId(fixture.subjectId),
  subjectDisplayName: decodeActivitySubjectName(fixture.subjectDisplayName),
  fields: Object.freeze({
    ...fixture.fields,
    capabilityStatus: ActivityCaptureCapabilityStatusSchema.parse(fixture.fields.capabilityStatus),
    adapterId: ActivityNetworkAdapterIdSchema.parse(fixture.fields.adapterId),
    networkProtocol: ActivityNetworkProtocolSchema.parse(fixture.fields.networkProtocol),
    tcpState: ActivityNetworkTcpStateSchema.parse(fixture.fields.tcpState),
    localIp: ActivityNetworkEndpointAddressSchema.parse(fixture.fields.localIp),
    destinationIp: ActivityNetworkEndpointAddressSchema.parse(fixture.fields.destinationIp),
    destinationDomain: ActivityNetworkDomainNameSchema.parse(fixture.fields.destinationDomain),
    domainAttributionStatus: ActivityDomainAttributionStatusSchema.parse(fixture.fields.domainAttributionStatus),
    processAttributionStatus: ActivityProcessAttributionStatusSchema.parse(fixture.fields.processAttributionStatus),
    processName: ActivityNetworkProcessNameSchema.parse(fixture.fields.processName),
  }),
});
