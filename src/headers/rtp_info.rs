use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

use uriparse::URI;

use std::{
    string::ToString,
    fmt::{
        Display,
        Error,
        Formatter,
    },
};

pub struct RTPInfoParameter<'a> {
    url: URI<'a>,
    ssrc: String,
    rtptime: u32,
    seq: u32,
}

impl<'a> RTPInfoParameter<'a> {
    fn new(url: URI<'a>, ssrc: String, rtptime: u32, seq: u32) -> RTPInfoParameter<'a> {
        RTPInfoParameter {
            url,
            ssrc,
            rtptime,
            seq,
        }
    }

    fn set_url(&mut self, url: URI<'a>) {
        self.url = url;
    }

    fn set_seq(&mut self, seq: u32) {
        self.seq = seq;
    }

    fn set_rtptime(&mut self, rtptime: u32) {
        self.rtptime = rtptime;
    }

    fn set_ssrc(&mut self, ssrc: String) {
        self.ssrc = ssrc;
    }
}

impl Display for RTPInfoParameter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "url=\"{}\" ssrc={}:seq={}; rtptime={}",
            self.url, self.ssrc,
            self.seq, self.rtptime
        )
    }
}

pub struct RTPInfo<'a> {
    parameters: Vec<RTPInfoParameter<'a>>
}

impl<'a> RTPInfo<'a> {
    fn new() -> RTPInfo<'a> {
        RTPInfo {
            parameters: vec![],
        }
    }

    fn from(parameters: Vec<RTPInfoParameter<'a>>) -> RTPInfo<'a> {
        RTPInfo {
            parameters
        }
    }

    fn add_parameter(&mut self, parameter: RTPInfoParameter<'a>) {
        self.parameters.push(parameter);
    }
}

impl Header for RTPInfo<'_> {
    fn header(&self) -> String {
        String::from("RTP-Info")
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Setup, RequestMethod::Play,
          RequestMethod::Pause, RequestMethod::GetParameter,
          RequestMethod::PlayNotify]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        let mut content = String::new();

        let mut peekable = self.parameters.iter().peekable();
        while let Some(param) = peekable.next() {
            content.push_str(&param.to_string());
            if let Some(_) = peekable.peek() {
                content.push_str("\r\n");
            }
        }

        content
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_uri() -> (URI<'static>, String) {
        use uriparse::Authority;
        use uriparse::Scheme;
        use uriparse::Path;
        use uriparse::URIBuilder;

        let authority = Authority::from_parts(
                None as Option<&str>,
                None as Option<&str>,
                "example.com",
                Some(554),
        ).unwrap();

        let path = Path::try_from("/ISAPI/Streaming/Channels/101").unwrap();

        (
                URIBuilder::new()
                    .with_scheme(Scheme::RTSP)
                    .with_authority(Some(authority))
                    .with_path(path)
                    .build().unwrap(),
                String::from("rtsp://example.com:554/ISAPI/Streaming/Channels/101")
        )
    }

    fn get_parameter() -> (RTPInfoParameter<'static>, String) {
        let (uri, uri_str) = get_test_uri();
        let ssrc = String::from("AAAAAA");
        let rtptime = 111111;
        let seq = 111111;
        (
                RTPInfoParameter::new(
                        uri.clone(),
                        ssrc.clone(),
                        rtptime,
                        seq),
                format!("url=\"{uri}\" ssrc={ssrc}:seq={seq}; rtptime={rtptime}")
        )
    }

    #[test]
    fn test_one_parameter() {
        let (first_parameter, first_test_str) = get_parameter();
        let ri_header = RTPInfo::from(vec![first_parameter]);

        assert_eq!(
                &ri_header.value(),
                &first_test_str,
        )
    }

    #[test]
    fn test_two_parameters() {
        let (first_parameter, first_test_str) = get_parameter();
        let (second_parameter, second_test_str) = get_parameter();

        let ri_header = RTPInfo::from(vec![first_parameter, second_parameter]);

        assert_eq!(
                &ri_header.value(),
                &format!("{first_test_str}\r\n{second_test_str}")
        )
    }
}