mod screen_capture_real_proof_support;

#[path = "../../screen-capture-adapter-generated/screen_capture_real_proof_impl.rs"]
mod screen_capture_real_proof_impl;

fn main() -> screen_capture_real_proof_support::ProofResult<()> {
    screen_capture_real_proof_impl::main()
}
