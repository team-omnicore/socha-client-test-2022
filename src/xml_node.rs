use std::collections::{HashMap, VecDeque};
use std::io::BufReader;
use std::net::TcpStream;
use xml::reader::XmlEvent;
use xml::EventReader;

#[derive(Debug)]
pub struct XmlNode {
    pub name: String,
    pub data: String,
    pub attributes: HashMap<String, Vec<String>>,
    pub children: Vec<XmlNode>,
}

impl XmlNode {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            data: String::new(),
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn child(&self, name: &str) -> Option<&XmlNode> {
        for child in self.children.iter() {
            if child.name.as_str() == name {
                return Some(child);
            }
        }
        None
    }

    pub fn read_from(xml_parser: &mut EventReader<BufReader<&TcpStream>>) -> Self {
        let mut node_stack: VecDeque<XmlNode> = VecDeque::new();
        let mut has_received_first = false;
        let mut final_node: Option<XmlNode> = None;

        loop {
            match xml_parser.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let mut node = XmlNode::new();
                    node.name = name.local_name;
                    for attribute in attributes {
                        let attrib_name = attribute.name.local_name;
                        if !node.attributes.contains_key(&attrib_name) {
                            node.attributes.insert(attrib_name.to_string(), Vec::new());
                        }
                        node.attributes
                            .get_mut(&attrib_name)
                            .unwrap()
                            .push(attribute.value.to_string());
                    }
                    node_stack.push_back(node);
                    has_received_first = true;
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    if node_stack.len() > 2 {
                        let child = node_stack.pop_back().expect("Unexpectedly found empty XML node stack while trying to pop off new child element");
                        let mut node = node_stack.pop_back().expect("Unexpectedly found empty XML node stack while trying to hook up new child element");
                        node.children.push(child);
                        node_stack.push_back(node);
                    } else if has_received_first {
                        final_node = Some(node_stack.pop_back().expect(
                            "Unexpectedly found empty XML node stack while trying to return node",
                        ));
                    }
                }
                Ok(XmlEvent::Characters(content)) => {
                    node_stack.back_mut().expect("Unexpectedly found empty XML node stack while trying to add characters").data += content.as_str();
                }
                Err(_) => {
                    break;
                }
                _ => {}
            }
            if final_node.is_some() {
                break;
            }
        }
        final_node.unwrap()
    }
}
