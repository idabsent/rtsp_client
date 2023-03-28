use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

use std::string::ToString;

pub struct Public {
    allowed: Vec<RequestMethod>,
}

impl Public {
    fn new() -> Public {
        Public {
            allowed: vec![],
        }
    }

    fn from(allowed: Vec<RequestMethod>) -> Public {
        Public {
            allowed,
        }
    }

    fn add_method(&mut self, method: RequestMethod) {
        self.allowed.push(method)
    }
}

impl Header for Public {
    fn header() -> String {
        String::from("Header")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Options,
          RequestMethod::Play, RequestMethod::Pause,
          RequestMethod::Setup, RequestMethod::Teardown,
          RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::Redirect]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::RequestResponse
    }

    fn value(&self) -> String {
        let mut content = String::new();

        let mut peekable = self.allowed.iter().peekable();
        while let Some(method) = peekable.next() {
            content.push_str(&method.to_string());
            if let Some(_) = peekable.peek() {
                content.push_str(", ")
            }
        }

        content
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one_public_method() {
        let p_header = Public::from(vec![RequestMethod::Describe]);
        assert_eq!(&p_header.value(), "DESCRIBE");
    }

    #[test]
    fn test_two_public_methods() {
        let p_header = Public::from(vec![RequestMethod::Describe, RequestMethod::Options]);
        assert_eq!(&p_header.value(), "DESCRIBE, OPTIONS");
    }
}