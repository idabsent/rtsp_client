use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub fn require_helper() -> String {
    String::from("Require")
}

pub struct Require {
    funcs: Vec<String>,
}

impl Require {
    fn new() -> Require {
        Require {
            funcs: vec![],
        }
    }

    fn from(funcs: Vec<String>) -> Require {
        Require {
            funcs
        }
    }

    fn add_required_func(&mut self, func: String) {
        self.funcs.push(func)
    }
}

impl Header for Require {
    fn header(&self) -> String {
        String::from("Require")
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Setup,
          RequestMethod::Play, RequestMethod::Options,
          RequestMethod::Pause, RequestMethod::Teardown,
          RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::Redirect, RequestMethod::PlayNotify]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::RequestResponse
    }

    fn value(&self) -> String {
        let mut content = String::new();

        let mut peekable = self.funcs.iter().peekable();
        while let Some(func) = peekable.next() {
            content.push_str(&func);
            if let Some(_) = peekable.peek() {
                content.push(' ');
            }
        }

        content
    }
}