import { redactStructuredLogValue } from './log-redaction';
import { assertBoundedText, MaximumStructuredDataBytes } from './logTextCustody';
import { assertStructuredLogBounds } from './structuredLogBounds';

export function serializeStructuredLogDataForCustody(value: unknown): string | null {
  if (value == null) {
    return null;
  }
  assertStructuredLogBounds(value);
  const serialized = JSON.stringify(redactStructuredLogValue(value));
  if (serialized == null) {
    throw new Error('structured log data is not serializable');
  }
  assertBoundedText(serialized, 'structured log data', MaximumStructuredDataBytes);
  return serialized;
}
