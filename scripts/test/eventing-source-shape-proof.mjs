import { collectSourceShapeReport } from '../check-source-shape.mjs';

const { findings, warnings } = collectSourceShapeReport();
const eventingFindings = findings.filter((entry) => entry.path.startsWith('crates/ocentra-eventing/'));
const eventingWarnings = warnings.filter((entry) => entry.path.startsWith('crates/ocentra-eventing/'));

if (eventingWarnings.length > 0) {
  console.log('Eventing source shape warnings: files/functions are near their size limits.');
  for (const warning of eventingWarnings) {
    console.log(`${warning.path}:${warning.line} ${warning.reason}`);
  }
}

if (eventingFindings.length > 0) {
  console.error('Eventing source shape guard failed. Split oversized eventing files/functions/classes before adding behavior.');
  for (const finding of eventingFindings) {
    console.error(`${finding.path}:${finding.line} ${finding.reason}`);
  }
  process.exit(1);
}

console.log('Eventing source shape guard passed.');
