use super::{
    tools::OrdHeadersBox,
    interface::{
        Request,
        RequestMethod,
        HeadersBox,
    },
};

pub struct Teardown {
    headers_box: OrdHeadersBox,
}

impl Teardown {
    pub fn new(headers_box: OrdHeadersBox) -> Teardown {
        Teardown {
            headers_box,
        }
    }
}

impl Request for Teardown {
    type HeaderItem = <OrdHeadersBox as HeadersBox>::HeaderItem;

    fn method(&self) -> RequestMethod {
        RequestMethod::Teardown
    }

    fn get_header(&self, header_helper: fn() -> String) -> Option<&Self::HeaderItem> {
        self.headers_box.get_header(header_helper)
    }
}