use crate::util::{SCResult, FromXmlNode, XmlNode};
use super::ScoreAggregation;

/// A single score fragment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreFragment {
    pub name: String,
    pub aggregation: ScoreAggregation,
    pub relevant_for_ranking: bool
}

impl FromXmlNode for ScoreFragment {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            name: node.attribute("name")?.to_owned(),
            aggregation: node.child_by_name("aggregation")?.content().parse()?,
            relevant_for_ranking: node.child_by_name("relevantForRanking")?.content().parse()?
        })
    }
}
