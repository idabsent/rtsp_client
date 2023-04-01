use super::{
    tools::OrdHeadersBox,
    interface::{
        Request,
        RequestMethod,
        HeadersBox,
    },
};

pub struct Setup {
    headers_box: OrdHeadersBox,
}

impl Setup {
    pub fn new(headers_box: OrdHeadersBox) -> Setup {
        Setup {
            headers_box,
        }
    }
}

impl Request for Setup {
    type HeaderItem = <OrdHeadersBox as HeadersBox>::HeaderItem;

    fn method(&self) -> RequestMethod {
        RequestMethod::Setup
    }

    fn get_header(&self, header_helper: fn() -> String) -> Option<&Self::HeaderItem> {
        self.headers_box.get_header(header_helper)
    }
}