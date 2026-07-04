pub(super) fn ratio_basis_points(numerator: usize, denominator: usize) -> Option<u16> {
    if denominator == 0 {
        return None;
    }
    let scaled = (numerator as u32 * 10_000 + denominator as u32 / 2) / denominator as u32;
    Some(scaled as u16)
}
