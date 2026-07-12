use ocentra_schema::typescript_literal::json_object_to_typescript_literal;

#[test]
fn apostrophes_select_double_quoted_typescript_strings() {
    assert_eq!(
        json_object_to_typescript_literal("{\n  \"notes\": \"workpack's claims\"\n}"),
        "{\n  notes: \"workpack's claims\",\n}"
    );
}

#[test]
fn long_string_properties_wrap_at_the_configured_width() {
    let source = "{\n  \"claimBoundary\": \"this is a deliberately long claim boundary that should wrap when rendered as a TypeScript property because the generated proof must match the repository formatter width\"\n}";
    let rendered = json_object_to_typescript_literal(source);
    assert_eq!(
        rendered,
        "{\n  claimBoundary:\n    'this is a deliberately long claim boundary that should wrap when rendered as a TypeScript property because the generated proof must match the repository formatter width',\n}"
    );
}

#[test]
fn short_scalar_arrays_compact() {
    assert_eq!(
        json_object_to_typescript_literal(
            "{\n  \"manualProofRequirements\": [\n    \"provider setup review required\"\n  ]\n}"
        ),
        "{\n  manualProofRequirements: ['provider setup review required'],\n}"
    );
    assert_eq!(
        json_object_to_typescript_literal(
            "{\n  \"rows\": [\n    {\n      \"manualProofRequirements\": [\n        \"provider setup review required\"\n      ]\n    }\n  ]\n}"
        ),
        "{\n  rows: [\n    {\n      manualProofRequirements: ['provider setup review required'],\n    },\n  ],\n}"
    );
}
