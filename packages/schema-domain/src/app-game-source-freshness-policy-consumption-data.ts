/* thin adapter over Rust-generated app-game preview/source-freshness fixture data */

import { AppGameSourceFreshnessPolicyConsumptionMatrixSchema as SchemaDomainAppGameSourceFreshnessPolicyConsumptionMatrixSchema } from '@ocentra-parent/schema-domain/app-game-source-freshness-policy-consumption';
import { evaluateAppGameSourceFreshnessPolicyReadiness } from './app-game-source-freshness-policy-consumption';
import { AppGameSourceFreshnessPolicyConsumptionMatrixId } from './app-game-source-freshness-policy-consumption-values';
import { ParentContractSchemaVersion } from './family-reference-primitives';
import {
  AppGameSourceFreshnessPolicyConsumptionGeneratedAtGenerated,
  AppGameSourceFreshnessPolicyConsumptionRequestsGenerated,
} from './generated/app-game-preview-source-freshness-data';

const GeneratedAt = AppGameSourceFreshnessPolicyConsumptionGeneratedAtGenerated;

export const AppGameSourceFreshnessPolicyConsumptionRequests =
  AppGameSourceFreshnessPolicyConsumptionRequestsGenerated;

export const AppGameSourceFreshnessPolicyConsumptionMatrix =
  SchemaDomainAppGameSourceFreshnessPolicyConsumptionMatrixSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    matrixId: AppGameSourceFreshnessPolicyConsumptionMatrixId,
    generatedAt: GeneratedAt,
    readiness: AppGameSourceFreshnessPolicyConsumptionRequests.map((request, index) =>
      evaluateAppGameSourceFreshnessPolicyReadiness(
        request,
        `source-freshness-policy-readiness-${index + 1}`,
        GeneratedAt
      )
    ),
  });
