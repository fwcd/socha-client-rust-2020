use std::str::FromStr;

/// Determines how scores should be aggregated (e.g. summed up or averaged over).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScoreAggregation {
    Sum,
    Average
}

impl FromStr for ScoreAggregation {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, String> {
        match raw {
            "SUM" => Ok(Self::Sum),
            "AVERAGE" => Ok(Self::Average),
            _ => Err(format!("Unknown score aggregation: {}", raw))
        }
    }
}
