# 26 - TabAgent Code Audit And Reuse Map

## Target State

TabAgent reference code is mapped to Ocentra-owned contracts before anything is
copied, extracted, or adapted.

## Where We Are

Local TabAgent and TabAgentServer files were inspected and indexed in
`tabagent-source-index.md`.

## Checklist

- [ ] Confirm current local TabAgent file list.
- [ ] Map native bridge code to Ocentra command/status contracts.
- [ ] Map model lifecycle code to Ocentra runtime/provider contracts.
- [ ] Map cache code to Ocentra model artifact/cache contracts.
- [ ] Map graph code to Ocentra source-cited memory/graph contracts.
- [ ] List non-reused UI/persona/remote/string ids.

## Proof

- Reuse map updated.
- No extracted code without contract and license note.
- No copied TabAgent string ids in app/runtime source.
