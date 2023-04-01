use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub fn media_range_helper() -> String {
    String::from("Media-Range")
}

pub struct MediaRange;

impl Header for MediaRange {
    fn header(&self) -> String {
        media_range_helper()
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
        todo!()
    }
}