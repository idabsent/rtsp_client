use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub fn supported_helper() -> String {
    String::from("Supported")
}

pub struct Supported {
    funcs: Vec<String>,
}

impl Supported {
    fn new(funcs: Vec<String>) -> Supported {
        Supported {
            funcs,
        }
    }

    fn add_func_tag(&mut self, tag: String) {
        self.funcs.push(tag)
    }
}

impl Header for Supported {
    fn header(&self) -> String {
        supported_helper()
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Setup, RequestMethod::Play,
          RequestMethod::Options, RequestMethod::Teardown,
          RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::Redirect, RequestMethod::Describe,
          RequestMethod::Pause,]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        let mut content = String::new();

        let mut peekable = self.funcs.iter().peekable();
        while let Some(tag) = peekable.next() {
            content.push_str(&tag);
            if let Some(_) = peekable.peek() {
                content.push_str(", ");
            }
        }

        content
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one_tag() {
        let s_header = Supported::new(vec!["first".to_string()]);
        assert_eq!(&s_header.value(), "first");
    }

    #[test]
    fn test_two_tag() {
        let s_header = Supported::new(vec!["first".to_string(), "second".to_string()]);
        assert_eq!(&s_header.value(), "first, second");
    }
}