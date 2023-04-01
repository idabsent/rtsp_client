use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub fn mtag_helper() -> String {
    String::from("MTag")
}

pub struct MTag;

impl Header for MTag {
    fn header(&self) -> String {
        mtag_helper()
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