use crate::requests::interface::RequestMethod;

pub enum HeaderPosition {
    General,
    RequestResponse,
    MessageBody,
}

pub trait Header {
    fn header(&self) -> String;
    fn allow_in_methods(&self) -> &'static [RequestMethod];
    fn header_position(&self) -> HeaderPosition;
    fn value(&self) -> String;
}