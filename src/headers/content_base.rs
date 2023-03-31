use uriparse::uri::URI;

use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::{RequestMethod},
};

pub struct ContentBase<'a> {
    uri: URI<'a>,
}

impl<'a> ContentBase<'a> {
    fn new(uri: URI) -> ContentBase {
        ContentBase {
            uri,
        }
    }

    fn set_uri(&mut self, uri: URI<'a>) {
        self.uri = uri;
    }
}

impl<'a> Header for ContentBase<'a> {
    fn header(&self) -> String {
        String::from("Content-Base")
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Options,
          RequestMethod::Setup, RequestMethod::GetParameter,
          RequestMethod::SetParameter, RequestMethod::Redirect,
          RequestMethod::PlayNotify, RequestMethod::Play,
          RequestMethod::Pause, RequestMethod::Teardown]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::MessageBody
    }

    fn value(&self) -> String {
        self.uri.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_uri() {
        use uriparse::URIBuilder;
        use uriparse::Scheme;
        use uriparse::Authority;
        use uriparse::Path;

        let authority = Authority::from_parts(
                None as Option<&str>,
                None as Option<&str>,
                "example.com",
                Some(554)
            ).unwrap();

        let path = Path::try_from("ISAPI/Streaming/Channels/101").unwrap();

        let uri = URIBuilder::new()
            .with_scheme(Scheme::RTSP)
            .with_authority(Some(authority))
            .with_path(path)
            .build().unwrap();

        let cb_header = ContentBase::new(uri);
        assert_eq!(&cb_header.value(), "rtsp://example.com:554/ISAPI/Streaming/Channels/101");
    }
}