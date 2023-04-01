use super::{
    tools::OrdHeadersBox,
    interface::{Request, RequestMethod, HeadersBox},
};

pub struct Describe {
    headers_box: OrdHeadersBox,
}

impl Describe {
    pub fn new(headers_box: OrdHeadersBox) -> Describe {
        Describe {
            headers_box,
        }
    }
}

impl Request for Describe {
    type HeaderItem = <OrdHeadersBox as HeadersBox>::HeaderItem;

    fn method(&self) -> RequestMethod {
        RequestMethod::Describe
    }

    fn get_header(&self, header_helper: fn() -> String) -> Option<&Self::HeaderItem> {
        self.headers_box.get_header(header_helper)
    }
}