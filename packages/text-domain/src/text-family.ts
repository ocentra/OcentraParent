import type { DisplayText } from './contracts';

export function resolveDisplayTextFromMap<Token extends string>(
  textMap: Readonly<Record<Token, DisplayText>>,
  token: Token,
  missingMessage: DisplayText
): DisplayText {
  const text = textMap[token];
  if (text === undefined) {
    throw new Error(missingMessage);
  }
  return text;
}
