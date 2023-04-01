use uriparse::URI;

use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub fn referrer_helper() -> String {
    String::from("Referrer")
}

pub struct Referrer<'a> {
    uri: URI<'a>
}

impl<'a> Referrer<'a> {
    fn new(uri: URI<'a>) -> Referrer<'a> {
        Referrer {
            uri,
        }
    }

    fn set_uri(&mut self, uri: URI<'a>) {
        self.uri = uri;
    }
}

impl Header for Referrer<'_> {
    fn header(&self) -> String {
        referrer_helper()
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::RequestResponse
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Play,
          RequestMethod::Pause, RequestMethod::Setup,
          RequestMethod::Options, RequestMethod::Teardown,
          RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::Redirect]
    }

    fn value(&self) -> String {
        self.uri.to_string()
    }
}