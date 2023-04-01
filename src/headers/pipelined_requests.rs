use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub fn pipelined_requests_helper() -> String {
    String::from("Pipelined-Requests")
}

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
    fn header(&self) -> String {
        pipelined_requests_helper()
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Options, RequestMethod::Setup,
          RequestMethod::Play, RequestMethod::Pause,
          RequestMethod::Teardown, RequestMethod::PlayNotify]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        self.request_id.to_string()
    }
}