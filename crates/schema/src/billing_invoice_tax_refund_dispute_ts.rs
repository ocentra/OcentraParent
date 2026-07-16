pub const BILLING_INVOICE_TAX_REFUND_DISPUTE_TYPESCRIPT: &str = concat!(
    include_str!("billing_invoice_tax_refund_dispute.template.txt"),
    include_str!("billing_invoice_tax_refund_dispute.row-guard-rules.template.txt"),
    include_str!("billing_invoice_tax_refund_dispute.row-boundary-rules.template.txt"),
    include_str!("billing_invoice_tax_refund_dispute.proof-read-model.template.txt"),
    include_str!("billing_invoice_tax_refund_dispute.proof-validation.template.txt"),
    include_str!("billing_invoice_tax_refund_dispute.lifecycle-factories.template.txt")
);
