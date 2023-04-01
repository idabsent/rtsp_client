use super::{
    tools::OrdHeadersBox,
    interface::{Request, RequestMethod, HeadersBox},
};

pub struct Redirect {
    headers_box: OrdHeadersBox,
}

impl Redirect {
    pub fn new(headers_box: OrdHeadersBox) -> Redirect {
        Redirect {
            headers_box,
        }
    }
}

impl Request for Redirect {
    type HeaderItem = <OrdHeadersBox as HeadersBox>::HeaderItem;

    fn method(&self) -> RequestMethod {
        RequestMethod::Redirect
    }

    fn get_header(&self, header_helper: fn() -> String) -> Option<&Self::HeaderItem> {
        self.headers_box.get_header(header_helper)
    }
}