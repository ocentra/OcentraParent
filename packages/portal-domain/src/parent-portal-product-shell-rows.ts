import type { ParentPortalRow } from './parent-portal-data';
import type { ParentPortalServiceStateInput } from './parent-portal-service-state';
import { productShellRowSpecs } from './parent-portal-product-shell-row-specs';
import { productShellSignals, type ProductShellSignal } from './parent-portal-product-shell-row-signals';

export function parentPortalProductShellRows(input: ParentPortalServiceStateInput): ParentPortalRow[] {
  const signals = productShellSignals(input);
  return productShellRowSpecs().map((spec) => rowFromSignal(spec, signals[spec.signalKind]));
}

function rowFromSignal(spec: ReturnType<typeof productShellRowSpecs>[number], signal: ProductShellSignal): ParentPortalRow {
  return {
    label: spec.label,
    order: spec.order,
    signalScore: signal.signalScore,
    readyCount: signal.readyCount,
    gapCount: signal.gapCount,
    primaryArea: spec.primaryArea,
    trend: signal.trend,
    tone: spec.tone,
  };
}
