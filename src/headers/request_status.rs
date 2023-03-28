use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub struct RequestStatus {
    cseq: u32,
    status_code: u16,
    reason: String,
}

impl RequestStatus {
    fn new(cseq: u32, status_code: u16, reason: String) -> RequestStatus {
        RequestStatus {
            cseq,
            status_code,
            reason
        }
    }
}

impl Header for RequestStatus {
    fn header() -> String {
        String::from("Request-Status")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::PlayNotify]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::RequestResponse
    }

    fn value(&self) -> String {
        format!("cseq={} status={}, reason=\"{}\"", self.cseq, self.status_code, self.reason)
    }
}