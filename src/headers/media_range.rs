use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub struct MediaRange;

impl Header for MediaRange {
    fn header() -> String {
        String::from("Media-Range")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Setup, RequestMethod::Play,
          RequestMethod::Pause, RequestMethod::GetParameter,
          RequestMethod::PlayNotify]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        todo!()
    }
}