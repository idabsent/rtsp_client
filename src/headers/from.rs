use uriparse::URI;

use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub fn from_helper() -> String {
    String::from("From")
}

pub struct From<'a> {
    uri: URI<'a>
}

impl<'a> From<'a> {
    fn new(uri: URI<'a>) -> From {
        From {
            uri,
        }
    }

    fn set_uri(&mut self, uri: URI<'a>) {
        self.uri = uri
    }
}

impl<'a> Header for From<'a> {
    fn header(&self) -> String {
        String::from("From")
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Setup,
          RequestMethod::Play, RequestMethod::Pause,
          RequestMethod::Teardown, RequestMethod::Options,
          RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::Redirect]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::RequestResponse
    }

    fn value(&self) -> String {
        self.uri.to_string()
    }
}