use std::string::ToString;

use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::{RequestMethod},
};

fn allow_helper() -> String {
    String::from("Allow")
}

pub struct Allow {
    allowed: Vec<RequestMethod>,
}

impl Allow {
    fn new() -> Allow {
        Allow {
            allowed: Vec::new(),
        }
    }

    fn add_method(&mut self, method: RequestMethod) {
        self.allowed.push(method);
    }
}

impl Header for Allow {
    fn header(&self) -> String  {
        allow_helper()
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod]  {
        &[RequestMethod::Describe, RequestMethod::Options,
          RequestMethod::SetParameter, RequestMethod::GetParameter]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::MessageBody
    }

    fn value(&self) -> String {
        let mut content = String::new();

        let mut peekable = self.allowed.iter().peekable();
        while let Some(method) = peekable.next() {
            content.push_str(&method.to_string());
            if let Some(_) = peekable.peek() {
                content.push(',');
            }
        }

        content
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_one_method() {
        let mut a_header = Allow::new();
        a_header.add_method(RequestMethod::GetParameter);
        assert_eq!(&a_header.value(), "GET_PARAMETER");
    }

    #[test]
    fn test_add_two_methods() {
        let mut a_header = Allow::new();
        a_header.add_method(RequestMethod::Play);
        a_header.add_method(RequestMethod::Pause);
        assert_eq!(&a_header.value(), "PLAY,PAUSE");
    }
}