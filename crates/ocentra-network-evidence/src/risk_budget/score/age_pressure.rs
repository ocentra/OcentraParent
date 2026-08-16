use super::super::*;

pub(super) fn age_pressure_points(age_band: NetworkRiskBudgetAgeBand) -> u32 {
    match age_band {
        NetworkRiskBudgetAgeBand::UnderTwelve => 15,
        NetworkRiskBudgetAgeBand::ThirteenToFifteen => 10,
        NetworkRiskBudgetAgeBand::SixteenToSeventeen => 5,
        NetworkRiskBudgetAgeBand::AdultOrUnknown => 0,
    }
}
