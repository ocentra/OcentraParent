# AI-21 Security Negative Proof

AI-21 rejects:

- unmanaged browser process classifications;
- non-YouTube generic web classifications;
- page body capture;
- transcript text capture;
- platform category/rating as policy authority;
- AI decision or policy decision claims from metadata.

The live proof harness also preserves the capture boundary:

- no generated or local fixture page is accepted as final evidence;
- raw YouTube watch-page HTML is not written to disk;
- raw title and description strings are redacted to hashes and lengths in proof
  artifacts;
- transcript text, cookies, tokens, local storage, account data, and browser
  secrets are not requested or persisted;
- metadata remains AI-input evidence only and does not become policy authority
  or enforcement proof.
