use uriparse::URI;

use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::{RequestMethod},
};

pub struct ContentLocation<'a> {
    location: URI<'a>,
}

impl<'a> ContentLocation<'a> {
    fn new(uri: URI<'a>) -> ContentLocation<'a> {
        ContentLocation {
            location: uri,
        }
    }

    fn set_location(&mut self, uri: URI<'a>) {
        self.location = uri;
    }
}

impl<'a> Header for ContentLocation<'a> {
    fn header() -> String {
        String::from("Content-Location")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Options,
          RequestMethod::Setup, RequestMethod::Play,
          RequestMethod::Pause, RequestMethod::Teardown,
          RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::Redirect, RequestMethod::PlayNotify]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::MessageBody
    }

    fn value(&self) -> String {
        self.location.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_content_location() {
        use uriparse::Scheme;
        use uriparse::Authority;
        use uriparse::URIBuilder;
        use uriparse::Path;

        let authority = Authority::from_parts(
                None as Option<&str>,
                None as Option<&str>,
                "example.com",
                Some(554)).unwrap();
        let path = Path::try_from("ISAPI/Streaming/Channels/101").unwrap();
        let uri = URIBuilder::new()
            .with_scheme(Scheme::RTSP)
            .with_authority(Some(authority))
            .with_path(path)
            .build().unwrap();

        let cl_header = ContentLocation::new(uri);
        assert_eq!(&cl_header.value(), "rtsp://example.com:554/ISAPI/Streaming/Channels/101");
    }
}