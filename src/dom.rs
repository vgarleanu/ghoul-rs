use std::collections::HashMap;
use std::collections::HashSet;

use crate::parser::Parser;

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element {
        tag: String,
        attrs: HashMap<String, String>,
    },
}

#[derive(Debug)]
pub struct DomNode {
    children: Vec<Self>,
    node_type: NodeType,
}

impl DomNode {
    pub fn parse(html: String) -> Self {
        let mut nodes = Parser::new(html).parse_nodes();

        if nodes.len() == 1 {
            nodes.swap_remove(0)
        } else {
            Self::element("html".into(), Default::default(), nodes)
        }
    }

    pub fn text(text: String) -> Self {
        Self {
            children: Vec::new(),
            node_type: NodeType::Text(text),
        }
    }

    pub fn element(tag: String, attrs: HashMap<String, String>, children: Vec<Self>) -> Self {
        Self {
            children,
            node_type: NodeType::Element { tag, attrs },
        }
    }
}

#[derive(Debug)]
pub struct ElementData {
    tag: String,
    attrs: HashMap<String, String>,
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attrs.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        self.attrs
            .get("class")
            .map_or_else(|| Default::default(), |x| x.split(" ").collect())
    }
}
