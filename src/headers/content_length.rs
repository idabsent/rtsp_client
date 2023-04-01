use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::{RequestMethod},
};

pub fn content_length_helper() -> String {
    String::from("Content-Length")
}

pub struct ContentLength {
    length: u32,
}

impl ContentLength {
    fn new(length: u32) -> ContentLength {
        ContentLength {
            length,
        }
    }

    fn set_length(&mut self, length: u32) {
        self.length = length;
    }
}

impl Header for ContentLength {
    fn header(&self) -> String {
        String::from("Content-Length")
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Options,
          RequestMethod::Setup, RequestMethod::Play,
          RequestMethod::Pause, RequestMethod::Teardown,
          RequestMethod::SetParameter, RequestMethod::GetParameter,
          RequestMethod::Redirect, RequestMethod::PlayNotify]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::MessageBody
    }

    fn value(&self) -> String {
        self.length.to_string()
    }
}