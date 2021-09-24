use std::collections::{HashMap, VecDeque};
use std::net::TcpStream;
use quick_xml::Reader;
use std::io::BufReader;
use quick_xml::events::Event;

pub struct XmlNode{
    pub name: String,
    pub text: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<XmlNode>,
}

impl XmlNode {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            text: String::new(),
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn child(&self, name: &str) -> Option<&XmlNode> {
        for child in &self.children {
            if child.name.as_str() == name {
                return Some(child);
            }
        }
        None
    }

    pub fn read_from(stream: &TcpStream) -> Self {

        let mut node_stack: VecDeque<XmlNode> = VecDeque::new();
        let mut has_received_first = false;
        let mut final_node: Option<XmlNode> = None;

        let mut buf = Vec::new();

        let mut reader = Reader::from_reader(BufReader::new(stream));
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let mut node = XmlNode::new();
                    node.name = String::from_utf8_lossy(e.name()).parse().unwrap();
                    for attribute_result in e.attributes() {
                        match attribute_result {
                            Ok(attribute) => unsafe {
                                let key: String = String::from_utf8_lossy(attribute.key).parse().unwrap();
                                let value: String = String::from_utf8_lossy(&*attribute.value).parse().unwrap();
                                if !node.attributes.contains_key(&key) {
                                    node.attributes.insert(key, value);
                                }
                            },
                            Err(_) => { panic!("Multiple attributes with same key") }
                        }
                    }

                    node_stack.push_back(node);
                    has_received_first = true;
                },
                Ok(Event::Empty(ref e)) => {
                    let mut node = XmlNode::new();
                    node.name = String::from_utf8_lossy(e.name()).parse().unwrap();

                    for attribute_result in e.attributes() {
                        match attribute_result {
                            Ok(attribute) => unsafe {
                                let key: String = String::from_utf8_lossy(attribute.key).parse().unwrap();
                                let value: String = String::from_utf8_lossy(&*attribute.value).parse().unwrap();
                                node.attributes.insert(key, value);
                            },
                            Err(_) => { panic!("Multiple attributes with same key") }
                        }
                    }
                    if node_stack.len() > 1 {
                        let mut parent_node = node_stack.pop_back().expect("Unexpectedly found empty XML node stack while trying to hook up new child element");
                        parent_node.children.push(node);
                        node_stack.push_back(parent_node);
                    }else{
                        final_node = Some(node_stack.pop_back().expect(
                            "Unexpectedly found empty XML node stack while trying to return node",
                        ));
                    }
                },
                Ok(Event::Text(ref e)) => {
                    if node_stack.len() > 1 {
                        let mut node = node_stack.pop_back().expect("Unexpectedly found empty XML node stack while trying to hook up new child element");
                        node.text = String::from_utf8_lossy(e.escaped()).parse().unwrap();
                        node_stack.push_back(node);
                    }
                },
                Ok(Event::End(ref e)) => {
                    if node_stack.len() > 2 {
                        let node = node_stack.pop_back().expect("Unexpectedly found empty XML node stack while trying to pop off new child element");
                        let mut parent_node = node_stack.pop_back().expect("Unexpectedly found empty XML node stack while trying to hook up new child element");
                        parent_node.children.push(node);
                        node_stack.push_back(parent_node);
                    } else if has_received_first {
                        final_node = Some(node_stack.pop_back().expect(
                            "Unexpectedly found empty XML node stack while trying to return node",
                        ));
                    }
                },
                Err(_) => panic!("Error"),
                _ => ()
            }
            if final_node.is_some() {
                break;
            }
        }
        final_node.unwrap()
    }
}