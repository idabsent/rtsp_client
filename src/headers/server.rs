use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub struct Server {
    server_name: String,
    version: f32,
}

impl Server {
    fn new(server_name: String, version: f32) -> Server {
        if version.is_sign_negative() {
            panic!("Version number must be positive number");
        }

        Server {
            server_name,
            version,
        }
    }

    fn set_server_name(&mut self, server_name: String) {
        self.server_name = server_name;
    }

    fn server_version(&mut self, version: f32) {
        self.version = version;
    }
}

impl Header for Server {
    fn header() -> String {
        String::from("Server")
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