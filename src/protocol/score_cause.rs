use std::str::FromStr;

/// Determines the cause of a game score.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScoreCause {
    Regular,
    Left,
    RuleViolation,
    SoftTimeout,
    HardTimeout,
    Unknown
}

impl FromStr for ScoreCause {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, String> {
        match raw {
            "REGULAR" => Ok(Self::Regular),
            "LEFT" => Ok(Self::Left),
            "RULE_VIOLATION" => Ok(Self::RuleViolation),
            "SOFT_TIMEOUT" => Ok(Self::SoftTimeout),
            "HARD_TIMEOUT" => Ok(Self::HardTimeout),
            "UNKNOWN" => Ok(Self::Unknown),
            _ => Err(format!("Unknown score cause: {}", raw))
        }
    }
}
