use crate::util::{SCResult, FromXmlNode, XmlNode};
use super::ScoreFragment;

/// The definition of a score.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreDefinition {
    pub fragments: Vec<ScoreFragment>
}

impl FromXmlNode for ScoreDefinition {
    fn from_node(node: &XmlNode) -> SCResult<Self> {
        Ok(Self {
            fragments: node.childs_by_name("fragment").map(ScoreFragment::from_node).collect::<SCResult<_>>()?
        })
    }
}
