use std::collections::{HashMap, VecDeque};
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};
use crate::util::SCError;

/// A deserialized, in-memory tree-representation
/// of an XML node.
#[derive(Debug, Default)]
pub struct XmlNode {
	name: String,
	data: String,
	attributes: HashMap<String, String>,
	childs: Vec<XmlNode>
}

impl XmlNode {
	/// Deserializes an XML node tree
	/// from the given XML event reader.
	pub fn read_from<R>(xml_parser: &mut EventReader<R>) -> Result<XmlNode, SCError> where R: Read {
		let mut node_stack = VecDeque::<XmlNode>::new();
		
		loop {
			match xml_parser.next() {
				Ok(XmlEvent::StartElement { name, attributes, .. }) => {
					let node = XmlNode {
						name: name.local_name,
						data: String::new(),
						attributes: attributes.iter().cloned().map(|attr| (attr.name.local_name, attr.value)).collect(),
						childs: Vec::new()
					};
					node_stack.push_back(node);
				},
				Ok(XmlEvent::EndElement { .. }) => {
					let node = node_stack.pop_back().ok_or_else(|| "Unexpectedly found empty XML node stack while popping off node".to_owned())?;
					if let Some(mut parent) = node_stack.pop_back() {
						parent.childs.push(node);
						node_stack.push_back(parent);
					} else {
						return Ok(node);
					}
				},
				Ok(XmlEvent::Characters(content)) => {
					let node = node_stack.back_mut().ok_or_else(|| "Unexpectedly found empty XML node stack while trying to add characters".to_owned())?;
					node.data += content.as_str();
				},
				Err(e) => return Err(e.into()),
				_ => ()
			}
		}
	}
}
