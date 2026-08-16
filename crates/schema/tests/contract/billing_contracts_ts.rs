use ocentra_schema::billing_contracts_ts::billing_contracts_typescript;

#[test]
fn billing_contracts_typescript_artifact_stays_checked_in() {
    let checked_in =
        include_str!("../../../../infra/cloudflare/src/generated/billing-contracts.ts");
    let generated = billing_contracts_typescript();
    let generated_lines: Vec<&str> = generated.lines().collect();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated_lines.first().copied(),
        Some("/* generated from crates/schema/src/billing_contracts_ts.rs */")
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line
                == "export const BillingCheckoutSessionRequestSchema = generatedSchema(")
            .count(),
        1
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line
                == "export const BillingSupportAdminRefundResultSchema = generatedSchema(")
            .count(),
        1
    );
    assert_eq!(generated.matches("passthroughSchema").count(), 0);
}
