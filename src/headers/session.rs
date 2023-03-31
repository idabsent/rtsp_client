use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub struct Session {
    session_id: String,
}

impl Session {
    fn new(session_id: String) -> Session {
        Session {
            session_id,
        }
    }

    fn set_session(&mut self, session_id: String) {
        self.session_id = session_id
    }
}

impl Header for Session {
    fn header(&self) -> String {
        String::from("Session")
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Setup, RequestMethod::Play,
          RequestMethod::Options, RequestMethod::Teardown,
          RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::Redirect, RequestMethod::PlayNotify,
          RequestMethod::Pause,]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        self.session_id.to_string()
    }
}