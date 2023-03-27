use uriparse::URI;

use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub struct Location<'a> {
    uri: URI<'a>,
}

impl<'a> Location<'a> {
    fn new(uri: URI<'a>) -> Location<'a> {
        Location {
            uri,
        }
    }

    fn set_uri(&mut self, uri: URI<'a>) {
        self.uri = uri;
    }
}

impl Header for Location<'_> {
    fn header() -> String {
        String::from("Location")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Setup,
          RequestMethod::Pause, RequestMethod::Play,
          RequestMethod::Options, RequestMethod::Teardown,
          RequestMethod::Redirect]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::RequestResponse
    }

    fn value(&self) -> String {
        self.uri.to_string()
    }
}