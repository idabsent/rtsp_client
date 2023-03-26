use crate::requests::interface::RequestMethod;

pub enum HeaderPosition {
    General,
    RequestResponse,
    MessageBody,
}

pub trait Header {
    fn header() -> String where Self: Sized;
    fn allow_in_methods() -> &'static [RequestMethod] where Self: Sized;
    fn header_position() -> HeaderPosition where Self: Sized;

    fn value(&self) -> String;
}