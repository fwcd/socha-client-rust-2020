use crate::util::{XmlNode, XmlNodeBuilder};
use super::{Field, AxialCoords, CubeCoords};

/// An owned field and a position.
/// 
/// If ownership over the field is not desired, you should
/// use a tuple instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionedField<C=AxialCoords> {
    pub field: Field,
    pub coords: C
}

impl<'a, C> From<PositionedField<C>> for XmlNodeBuilder<'a> where C: Into<CubeCoords> {
    fn from(field: PositionedField<C>) -> Self {
        let cube_coords = field.coords.into();
        XmlNodeBuilder::default()
            .attribute("class", "field")
            .attribute("x", cube_coords.x().to_string())
            .attribute("y", cube_coords.y().to_string())
            .attribute("z", cube_coords.z().to_string())
            .attribute("isObstructed", field.field.is_obstructed().to_string())
            .childs(field.field.piece_stack().iter().map(|&p| XmlNode::from(p)))
    }
}
