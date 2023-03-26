use std::string::ToString;

use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::{RequestMethod},
};

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
    fn header() -> String where Self: Sized {
        String::from("Allow")
    }

    fn allow_in_methods() -> &'static [RequestMethod] where Self: Sized {
        &[RequestMethod::Describe, RequestMethod::Options,
          RequestMethod::SetParameter, RequestMethod::GetParameter]
    }

    fn header_position() -> HeaderPosition where Self: Sized {
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