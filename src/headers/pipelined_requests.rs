use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub struct PipelinedRequests{
    request_id: u32,
}

impl PipelinedRequests {
    fn new(request_id: u32) -> PipelinedRequests {
        PipelinedRequests {
            request_id
        }
    }

    fn set_session(&mut self, request_id: u32) {
        self.request_id = request_id
    }
}

impl Header for PipelinedRequests {
    fn header() -> String {
        String::from("Pipelined-Request")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Options, RequestMethod::Setup,
          RequestMethod::Play, RequestMethod::Pause,
          RequestMethod::Teardown, RequestMethod::PlayNotify]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        self.request_id.to_string()
    }
}