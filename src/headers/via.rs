use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

use std::{
    string::ToString,
    fmt::{
        Display,
        Error,
        Formatter,
    },
};

pub struct ViaParameter {
    protocol_version: f32,
    sent_by: String,
}

impl Display for ViaParameter {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:.1} {}", self.protocol_version, self.sent_by)
    }
}

impl ViaParameter {
    fn new(protocol_version: f32, sent_by: String) -> ViaParameter {
        ViaParameter {
            protocol_version,
            sent_by,
        }
    }
}

pub struct Via {
    nodes: Vec<ViaParameter>,
}

impl Via {
    fn new() -> Via {
        Via {
            nodes: vec![],
        }
    }

    fn from(nodes: Vec<ViaParameter>) -> Via {
        Via {
            nodes,
        }
    }

    fn add_node(&mut self, node: ViaParameter) {
        self.nodes.push(node);
    }
}

impl Header for Via {
    fn header() -> String {
        String::from("Via")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Setup,
          RequestMethod::Play, RequestMethod::Options,
          RequestMethod::Pause, RequestMethod::Teardown,
          RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::Redirect, RequestMethod::PlayNotify]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        let mut content = String::new();

        let mut peekable = self.nodes.iter().peekable();
        while let Some(node) = peekable.next() {
            content.push_str(&node.to_string());
            if let Some(node) = peekable.peek() {
                content.push_str(", ");
            }
        }

        content
    }
}