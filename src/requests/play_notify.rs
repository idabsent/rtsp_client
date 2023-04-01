use super::{
    tools::OrdHeadersBox,
    interface::{Request, RequestMethod, HeadersBox},
};

pub struct PlayNotify {
    headers_box: OrdHeadersBox,
}

impl PlayNotify {
    pub fn new(headers_box: OrdHeadersBox) -> PlayNotify {
        PlayNotify {
            headers_box,
        }
    }
}

impl Request for PlayNotify {
    type HeaderItem = <OrdHeadersBox as HeadersBox>::HeaderItem;

    fn method(&self) -> RequestMethod {
        RequestMethod::PlayNotify
    }

    fn get_header(&self, header_helper: fn() -> String) -> Option<&Self::HeaderItem> {
        self.headers_box.get_header(header_helper)
    }
}