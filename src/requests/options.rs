use super::{
    tools::OrdHeadersBox,
    interface::{Request, RequestMethod, HeadersBox},
};

pub struct Options {
    headers_box: OrdHeadersBox,
}

impl Options {
    pub fn new(headers_box: OrdHeadersBox) -> Options {
        Options {
            headers_box,
        }
    }
}

impl Request for Options {
    type HeaderItem = <OrdHeadersBox as HeadersBox>::HeaderItem;

    fn method(&self) -> RequestMethod {
        RequestMethod::Options
    }

    fn get_header(&self, header_helper: fn() -> String) -> Option<&Self::HeaderItem> {
        self.headers_box.get_header(header_helper)
    }
}