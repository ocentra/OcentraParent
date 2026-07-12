import type { BrowserGameParsedUrl } from './browser_game_url_shape_evaluator';

export function protocolShapeFor(parsedUrl: BrowserGameParsedUrl) {
  if (parsedUrl.protocol === 'http:' || parsedUrl.protocol === 'https:') {
    return 'http-family';
  }
  return parsedUrl.protocol.length > 0 ? 'non-http' : 'missing';
}

export function hostShapeFor(parsedUrl: BrowserGameParsedUrl) {
  const hostname = parsedUrl.hostname.toLowerCase();
  if (hostname === 'localhost') {
    return 'localhost-like';
  }
  if (/^\d{1,3}(?:\.\d{1,3}){3}$/.test(hostname)) {
    return 'ip-like';
  }
  return hostname.includes('.') ? 'domain-like' : 'unknown';
}
