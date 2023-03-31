use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub struct MTag;

impl Header for MTag {
    fn header(&self) -> String {
        String::from("MTag")
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Setup,
          RequestMethod::GetParameter]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::MessageBody
    }

    fn value(&self) -> String {
        todo!()
    }
}