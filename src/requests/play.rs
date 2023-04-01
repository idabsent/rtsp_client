use super::{
    tools::OrdHeadersBox,
    interface::{Request, RequestMethod, HeadersBox},
};

pub struct Play {
    headers_box: OrdHeadersBox,
}

impl Play {
    pub fn new(headers_box: OrdHeadersBox) -> Play {
        Play {
            headers_box,
        }
    }
}

impl Request for Play {
    type HeaderItem = <OrdHeadersBox as HeadersBox>::HeaderItem;

    fn method(&self) -> RequestMethod {
        RequestMethod::Play
    }

    fn get_header(&self, header_helper: fn() -> String) -> Option<&Self::HeaderItem>
    {
        self.headers_box.get_header(header_helper)
    }
}