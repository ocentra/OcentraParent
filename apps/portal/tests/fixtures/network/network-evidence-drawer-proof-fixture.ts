import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

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
  fields: Object.freeze({
    ...fixture.fields,
  }),
});
