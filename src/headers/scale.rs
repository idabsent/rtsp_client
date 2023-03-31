use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub struct Scale {
    scale: i8,
}

impl Scale {
    fn new(scale: i8) -> Scale {
        Scale {
            scale,
        }
    }

    fn set_scale(&mut self, scale: i8) {
        self.scale = scale;
    }
}

impl Header for Scale {
    fn header(&self) -> String {
        String::from("Scale")
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Setup, RequestMethod::Play,
          RequestMethod::Options, RequestMethod::GetParameter,
          RequestMethod::Redirect, RequestMethod::PlayNotify]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        self.scale.to_string()
    }
}