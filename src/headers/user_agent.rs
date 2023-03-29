use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub struct UserAgent {
    agent_name: String,
    version: f32,
}

impl UserAgent {
    fn new(agent_name: String, version: f32) -> Server {
        if version.is_sign_negative() {
            panic!("Version number must be positive number");
        }

        UserAgent {
            agent_name,
            version,
        }
    }

    fn set_server_name(&mut self, agent_name: String) {
        self.agent_name = agent_name;
    }

    fn server_version(&mut self, version: f32) {
        self.version = version;
    }
}

impl Header for Server {
    fn header() -> String {
        String::from("User-Agent")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Setup,
          RequestMethod::Play, RequestMethod::Options,
          RequestMethod::Pause, RequestMethod::Teardown,
          RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::Redirect, RequestMethod::PlayNotify]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        format!("{}/{}", self.server_name, self.version)
    }
}